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
    /// Check the current token without advancing
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
    /// Get the next token and advance the cursor
    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.index);
        if tok.is_some() {
            self.index += 1;
        }
        tok
    }
    /// Check if there are more tokens
    pub fn has_more(&self) -> bool {
        self.index < self.tokens.len()
    }
    /// Reset the parser to the beginning
    pub fn seek_top(&mut self) {
        self.index = 0;
    }
    /// Get the current index
    pub fn get_index(&self) -> usize {
        self.index
    }
    /// Set the current index
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
    /// Test if the current token matches the given kind
    pub fn test_kind(&self, kind: TokenKind) -> bool {
        if let Some(tok) = self.peek() {
            return tok.kind == kind;
        }
        false
    }
    /// Test if the current token matches any of the given kinds
    pub fn test_kinds(&self, kinds: &[TokenKind]) -> bool {
        if let Some(tok) = self.peek() {
            return kinds.contains(&tok.kind);
        }
        false
    }
    /// トークンの並びが、[A, B, C...]の形になっているかテストする
    pub fn test_kind_array(&self, kinds: &[TokenKind]) -> bool {
        // サイズを確認
        if self.index + kinds.len() > self.tokens.len() {
            return false;
        }
        // 各トークンを確認
        for (i, kind) in kinds.iter().enumerate() {
            if self.tokens[self.index + i].kind != *kind {
                return false;
            }
        }
        return true;
    }
    /// Test if the current token is an operator
    pub fn test_operator(&self) -> bool {
        if let Some(tok) = self.peek() {
            return tok.kind.is_operator();
        }
        false
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
fn parse_sentence(parser: &mut Parser, parent: &mut AstNode) -> bool {
    // Nop?
    if parser.test_kind(TokenKind::Nop) {
        return parse_nop(parser, parent);
    }
    // コメント?
    if parser.test_kind(TokenKind::Comment) {
        return parse_comment(parser, parent);
    }
    // 変数 = 式の場合
    if parser.test_kind_array(&[TokenKind::Word, TokenKind::Eq]) {
        return parse_let(parser, parent);
    }
    // 一般的な文
    {      
        while parse_value(parser) {
            // 値の解析が成功した場合、次のトークンへ
        }
        // 特殊文の解析 - 現在のトークンをチェック
        if parse_print(parser, parent) {
            return true;
        }
    }
    // EOS?
    if parser.test_kind(TokenKind::EOS) {
        return parse_eos(parser, parent);
    }
    // 不明なトークン
    if parser.has_more() {
        let t = parser.peek().unwrap();
        println!("[ERROR][Parser] Unknown token at {}:{}: {:?}", 
            t.pos.line, t.pos.column, t);
    }
    false
}

fn parse_let(parser: &mut Parser, parent: &mut AstNode) -> bool {
    if let Some(word_t) = parser.next() {
        let word_t = word_t.clone();
        if let Some(eq_t) = parser.next() {
            let eq_t = eq_t.clone();
            // 変数名ノード
            let mut var_node = AstNode::new_pos(AstKind::Nop, word_t.pos);
            if let Some(ref val) = word_t.value {
                let var_name_s = val.clone();
                var_node.value = crate::value::Value::from_string(var_name_s);
            }
            // 右辺の式(1つ)を解析
            if !parse_value(parser) {
                // 値の解析が成功した場合、次のトークンへ
                println!("[ERROR][Parser] Expected expression after '=' at {}:{}", 
                    eq_t.pos.line, eq_t.pos.column);
                return false;
            }
            // スタックから右辺のASTノードを取り出す
            if let Some(expr_node) = parser.stack.pop() {
                // 代入ノードを作成
                let mut let_node = AstNode::new_pos(AstKind::Let, word_t.pos);
                let_node.add_child(var_node);
                let_node.add_child(expr_node);
                parent.add_child(let_node);
                return true;
            } else {
                println!("[ERROR][Parser] Expected expression after '=' at {}:{}", 
                    eq_t.pos.line, eq_t.pos.column);
            }
        }
    }
    false
}

fn parse_value(parser: &mut Parser) -> bool {
    if let Some(t) = parser.peek() {
        let kind = t.kind;  // Copy trait なのでコストゼロ
        return match kind {
            TokenKind::Number => parse_number(parser),
            TokenKind::Str => parse_str(parser),
            TokenKind::Word => parse_word(parser),
            TokenKind::ParenL => parse_parenthesis(parser),
            _ => false,
        };
    }
    false
}

fn parse_nop(_parser: &mut Parser, _parent: &mut AstNode) -> bool {
    true
}

fn parse_comment(parser: &mut Parser, parent: &mut AstNode) -> bool {
    if let Some(t) = parser.next() {
        let mut node = AstNode::new_pos(AstKind::Comment, t.pos);
        if let Some(ref val) = t.value {
            node.value = crate::value::Value::from_string(val.clone());
        }
        parent.add_child(node);
        return true;
    }
    false
}

fn parse_number(parser: &mut Parser) -> bool {
    if let Some(token) = parser.next() {
        let token = token.clone();
        let has_josi = push_value_to_stack(parser, &token);
        // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
        if !has_josi {
            process_operators(parser);
        }
        return true;
    }
    false
}

fn parse_str(parser: &mut Parser) -> bool {
    if let Some(token) = parser.next() {
        let token = token.clone();
        let has_josi = push_value_to_stack(parser, &token);
        // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
        if !has_josi {
            process_operators(parser);
        }
        return true;
    }
    false
}

fn parse_word(parser: &mut Parser) -> bool {
    if let Some(token) = parser.next() {
        let token = token.clone();
        let has_josi = push_value_to_stack(parser, &token);
        // 助詞がない場合のみ、次のトークンをpeek()して演算子なら処理
        if !has_josi {
            process_operators(parser);
        }
        return true;
    }
    false
}

fn parse_print(parser: &mut Parser, parent: &mut AstNode) -> bool {
    if parser.test_kind(TokenKind::Print) == false {
        return false;
    }
    let t = parser.next().unwrap();
    let mut node = AstNode::new_pos(AstKind::Print, t.pos);
    if let Some(arg) = parser.stack.pop() {
        node.add_child(arg);
    } else {
        node.add_child(AstNode::new_nop());
    }
    parent.add_child(node);
    true
}

fn parse_eos(parser: &mut Parser, parent: &mut AstNode) -> bool {
    if let Some(t) = parser.next() {
        let node = AstNode::new_pos(AstKind::EOS, t.pos);
        parent.add_child(node);
        return true;
    }
    false
}

fn parse_parenthesis(parser: &mut Parser) -> bool {
    let start_token = parser.next().unwrap().clone();
    // 括弧内の式を解析するために一時的なトークン列を収集
    let mut paren_tokens = Vec::new();
    let mut paren_level = 1;
    
    while let Some(token) = parser.next() {
        let token = token.clone();
        match token.kind {
            TokenKind::ParenL => {
                paren_level += 1;
                paren_tokens.push(token);
            },
            TokenKind::ParenR => {
                paren_level -= 1;
                if paren_level == 0 {
                    // 括弧を閉じたので、括弧内の式を解析
                    let mut sub_parser = Parser::new(paren_tokens);
                    let mut dummy_parent = AstNode::new(AstKind::Node);
                    parse_sentence(&mut sub_parser, &mut dummy_parent);
                    
                    // 括弧内の式の結果をスタックに移す
                    if let Some(result) = sub_parser.stack.pop() {
                        parser.stack.push(result);
                    } else if let Some(children) = dummy_parent.children {
                        // スタックが空の場合、親ノードの最初の子を使う
                        if !children.is_empty() {
                            parser.stack.push(children[0].clone());
                        }
                    }
                    
                    // 括弧の後に演算子がある場合の処理
                    // 次のトークンをチェック
                    if let Some(next_token) = parser.peek() {
                        if next_token.kind.is_operator() {
                            process_operators(parser);
                        }
                    }
                    
                    return false;
                }
                paren_tokens.push(token);
            },
            _ => {
                paren_tokens.push(token);
            }
        }
    }
    
    // 括弧が閉じられなかった場合のエラー
    println!("[ERROR][Parser] Unmatched parenthesis at {}:{}", 
        start_token.pos.line, start_token.pos.column);
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
            let mut node = AstNode::new_pos(AstKind::Variable, pos);
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
        
        // 次の値を読む - 括弧の場合は parse_value で処理
        if let Some(next_value_token) = parser.peek() {
            if next_value_token.kind == TokenKind::ParenL {
                // 括弧を処理
                parse_value(parser);
            } else {
                // 通常の値を読む
                if let Some(token) = parser.next() {
                    let token = token.clone();
                    let has_josi = push_value_to_stack(parser, &token);
                    
                    // 助詞がある場合は、これ以上演算子を処理せずに現在の演算子を処理して終了
                    if has_josi {
                        process_single_operator(parser, &op_token);
                        return;
                    }
                } else {
                    return;
                }
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

