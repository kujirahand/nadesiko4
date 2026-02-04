pub mod lexer;
pub mod parser;
pub mod bytecode;
pub mod vm;
pub mod token;
pub mod ast;
pub mod ast_to_bytecode;
pub mod error;
pub mod source;
pub mod value;
pub mod char_type;

use crate::bytecode::NakoSystem;

/// Options for Nako4 compiler and VM
pub struct NakoOptions {
    pub is_debug: bool,
}
impl NakoOptions {
    pub fn new() -> Self {
        NakoOptions {
            is_debug: false,
        }
    }
}

pub fn hello() -> String {
    return "hello".to_string();
}

/// Returns the current version of the nadesiko4 crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Compile the given source code into VM code.
pub fn compile(source: &str, options: &NakoOptions) -> NakoSystem {
    let mut src = source::Source::new(source);
    // lex
    let tokens = lexer::lex(&mut src);
    if options.is_debug {
        for token in &tokens {
            println!("Token: kind={:?}, value={:?}, pos=({:?})",
                token.kind,
                token.value,
                token.pos,
            );
        }
    }
    // parse
    let ast = parser::parse(tokens);
    if options.is_debug {
        ast.print_tree(0);
    }
    // bytecode
    let sys = ast_to_bytecode::ast_to_bytecodes(&ast);
    if options.is_debug {
        for (i, code) in sys.codes.iter().enumerate() {
            println!("ByteCode[{}]: kind={:?}, arg1={}, arg2={}, arg3={}",
                i,
                code.kind,
                code.arg1,
                code.arg2,
                code.arg3,
            );
        }
    }
    sys
}

/// Execute easy for test and simple usage.
pub fn run_easy(source: &str, options: &NakoOptions) -> String {
    let mut sys = compile(source, options);
    vm::run(&mut sys);
    if sys.error_msg.len() > 0 {
        return sys.error_msg;
    }
    sys.output
}
