/// parser module
use crate::token::{Token, TokenKind};
use crate::ast::{AstNode, AstKind};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    stack: Vec<AstNode>,
}
impl Parser {
    /// Create a new parser instance
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            index: 0,
            stack: Vec::new(),
        }
    }
    // Check the current token without advancing
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
    // Get the next token and advance the cursor
    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.index);
        if tok.is_some() {
            self.index += 1;
        }
        tok
    }
    // Check if there are more tokens
    pub fn has_more(&self) -> bool {
        self.index < self.tokens.len()
    }
}

/// Parse the list of tokens into an AST.
pub fn parse(tokens: Vec<Token>) -> AstNode {
    let mut parser = Parser::new(tokens);
    let mut root = AstNode::new(AstKind::Node);
    parse_sentences(&mut parser, &mut root);
    // スタックに余剰があればエラーにする
    if !parser.stack.is_empty() {
        println!("[ERROR][Parser] Stack is not empty after parsing: {:?}", parser.stack);
    }
    return root;
}

/// 複数文の解析
fn parse_sentences(parser: &mut Parser, parent: &mut AstNode) -> bool {
    while parser.has_more() {
        parse_sentence(parser, parent);
    }
    true
}

/// 短文の解析
fn parse_sentence(parser: &mut Parser, parent: &mut AstNode) {
    while let Some(token) = parser.next() {
        let token = token.clone();
        let need_break = match token.kind {
            TokenKind::Nop => parse_nop(parser, parent, &token),
            TokenKind::Comment => parse_comment(parser, parent, &token),
            TokenKind::Number => parse_number(parser, parent, &token),
            TokenKind::Str => parse_str(parser, parent, &token),
            TokenKind::Word => parse_word(parser, parent, &token),
            TokenKind::Print => parse_print(parser, parent, &token),
            TokenKind::EOS => parse_eos(parser, parent, &token),
            _ if token.kind.is_operator() => {
                // 演算子は単独では処理せず、スキップ
                // 実際の処理は次の値を読んだ時に行う
                false
            },
            _ => parse_unknown(parser, parent, &token),
        };
        if need_break {
            break;
        }
    }
}

fn parse_nop(_parser: &mut Parser, _parent: &mut AstNode, _token: &Token) -> bool {
    // nop
    false
}

fn parse_comment(_parser: &mut Parser, parent: &mut AstNode, token: &Token) -> bool {
    let pos = token.pos;
    let mut node = AstNode::new_pos(AstKind::Comment, pos);
    if let Some(ref val) = token.value {
        node.value = crate::value::Value::from_string(val.clone());
    }
    parent.add_child(node);
    false
}

fn parse_number(parser: &mut Parser, _parent: &mut AstNode, token: &Token) -> bool {
    let has_josi = push_value_to_stack(parser, token);
    // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
    if !has_josi {
        process_operators(parser);
    }
    false
}

fn parse_str(parser: &mut Parser, _parent: &mut AstNode, token: &Token) -> bool {
    let has_josi = push_value_to_stack(parser, token);
    // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
    if !has_josi {
        process_operators(parser);
    }
    false
}

fn parse_word(parser: &mut Parser, _parent: &mut AstNode, token: &Token) -> bool {
    let has_josi = push_value_to_stack(parser, token);
    // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
    if !has_josi {
        process_operators(parser);
    }
    false
}

fn parse_print(parser: &mut Parser, parent: &mut AstNode, token: &Token) -> bool {
    let pos = token.pos;
    let mut node = AstNode::new_pos(AstKind::Print, pos);
    if let Some(arg) = parser.stack.pop() {
        node.add_child(arg);
    } else {
        node.add_child(AstNode::new_nop());
    }
    parent.add_child(node);
    true
}

fn parse_eos(_parser: &mut Parser, parent: &mut AstNode, token: &Token) -> bool {
    let mut node = AstNode::new(AstKind::EOS);
    node.pos = token.pos;
    parent.add_child(node);
    true
}

fn parse_unknown(_parser: &mut Parser, _parent: &mut AstNode, token: &Token) -> bool {
    println!("[ERROR][Parser] Unknown token: {:?}", token);
    false
}

/// 値をスタックに積む共通処理
/// 戻り値: 助詞がある場合はtrue、ない場合はfalse
fn push_value_to_stack(parser: &mut Parser, token: &Token) -> bool {
    let pos = token.pos;
    let has_josi = token.josi.is_some();
    
    match token.kind {
        TokenKind::Number => {
            let mut node = AstNode::new_pos(AstKind::Number, pos);
            if let Some(ref val_str) = token.value {
                if let Ok(num) = val_str.parse::<f64>() {
                    node.value = crate::value::Value::from_number(num);
                }
            }
            parser.stack.push(node);
        },
        TokenKind::Str => {
            let mut node = AstNode::new_pos(AstKind::String, pos);
            if let Some(ref val) = token.value {
                node.value = crate::value::Value::from_string(val.clone());
            }
            parser.stack.push(node);
        },
        TokenKind::Word => {
            let mut node = AstNode::new_pos(AstKind::Nop, pos);
            if let Some(ref val) = token.value {
                node.value = crate::value::Value::from_string(val.clone());
            }
            parser.stack.push(node);
        },
        _ => {
            println!("[ERROR][Parser] Cannot push non-value token to stack: {:?}", token);
            return false;
        }
    }
    
    has_josi
}

/// 演算子の優先順位を返す（数値が大きいほど優先度が高い）
fn get_operator_precedence(kind: TokenKind) -> i32 {
    match kind {
        TokenKind::Mul | TokenKind::Div => 2,
        TokenKind::Plus | TokenKind::Minus => 1,
        _ => 0,
    }
}

/// 次のトークンをpeek()して演算子なら優先順位を考慮して処理
fn process_operators(parser: &mut Parser) {
    while let Some(next_token) = parser.peek() {
        if !next_token.kind.is_operator() {
            break;
        }
        
        let op_token = parser.next().unwrap().clone();
        let op_precedence = get_operator_precedence(op_token.kind);
        
        // 次の値を読む
        if let Some(next_value_token) = parser.next() {
            let next_value_token = next_value_token.clone();
            
            // 値をスタックに積む
            let has_josi = push_value_to_stack(parser, &next_value_token);
            
            // 助詞がある場合は、これ以上演算子を処理せずに現在の演算子を処理して終了
            if has_josi {
                process_single_operator(parser, &op_token);
                return;
            }
            
            // さらに次の演算子をチェックして優先順位を比較
            if let Some(next_op_token) = parser.peek() {
                if next_op_token.kind.is_operator() {
                    let next_op_precedence = get_operator_precedence(next_op_token.kind);
                    
                    // 次の演算子の方が優先順位が高い場合は、先に処理
                    if next_op_precedence > op_precedence {
                        process_operators(parser);
                    }
                }
            }
        } else {
            println!("[ERROR][Parser] Expected value after operator at {}:{}", 
                op_token.pos.line, op_token.pos.column);
            return;
        }
        
        // 現在の演算子を処理
        process_single_operator(parser, &op_token);
    }
}

/// 単一の演算子を処理する（スタックから2つの値を取り出して演算ノードを作成）
fn process_single_operator(parser: &mut Parser, op_token: &Token) {
    let pos = op_token.pos;
    let right = parser.stack.pop();
    let left = parser.stack.pop();
    
    if left.is_none() || right.is_none() {
        println!("[ERROR][Parser] Operator requires two operands at {}:{}", pos.line, pos.column);
        return;
    }
    
    let ast_kind = match op_token.kind {
        TokenKind::Plus => AstKind::Plus,
        TokenKind::Minus => AstKind::Minus,
        TokenKind::Mul => AstKind::Mul,
        TokenKind::Div => AstKind::Div,
        _ => {
            println!("[ERROR][Parser] Unknown operator: {:?}", op_token.kind);
            return;
        }
    };
    
    let mut op_node = AstNode::new_pos(ast_kind, pos);
    op_node.add_child(left.unwrap());
    op_node.add_child(right.unwrap());
    
    parser.stack.push(op_node);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourcePos;
    use crate::value::Value;

    /// Ensures a simple expression is parsed into a Print node with a Plus child
    #[test]
    fn parse_simple_expression() {
        let pos = SourcePos::zero();
        let tokens = vec![
            Token::new(TokenKind::Number, Some("3".to_string()), pos),
            Token::new(TokenKind::Plus, None, pos),
            Token::new_arg(TokenKind::Number, "5", "を", pos),
            Token::new(TokenKind::Print, None, pos),
            Token::new(TokenKind::EOS, None, pos),
        ];

        let ast = parse(tokens);

        let root_children = ast.children.as_ref().expect("root should have children");
        assert!(root_children.len() >= 1, "root must contain one statement");

        let print_node = &root_children[0];
        assert_eq!(print_node.kind, AstKind::Print);

        let print_args = print_node.children.as_ref().expect("print should have an argument");
        assert!(print_args.len() >= 1, "print should have one argument");

        let plus_node = &print_args[0];
        assert_eq!(plus_node.kind, AstKind::Plus);

        let operands = plus_node.children.as_ref().expect("plus should have two operands");
        assert_eq!(operands.len(), 2);
        assert_eq!(operands[0].value, Value::from_number(3.0));
        assert_eq!(operands[1].value, Value::from_number(5.0));
    }
}

