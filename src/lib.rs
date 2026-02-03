pub mod lexer;
pub mod parser;
pub mod vmcode;
pub mod vm;
pub mod token;
pub mod ast;
pub mod ast_to_vmcode;
pub mod error;
pub mod source;
pub mod value;

use crate::vmcode::VmSystem;

pub fn hello() -> String {
    return "hello".to_string();
}

/// Returns the current version of the nadesiko4 crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Compile the given source code into VM code.
pub fn compile(source: &str) -> vmcode::VmSystem {
    let mut src = source::Source::new(source);
    let tokens = lexer::lex(&mut src);
    let ast = parser::parse(tokens);
    ast_to_vmcode::ast_to_vmcode(&ast)
}

/// Run the given source code.
pub fn run_from_str(source: &str) -> VmSystem {
    let mut sys = compile(source);
    vm::run(&mut sys);
    sys
}

/// Execute easy for test and simple usage.
pub fn run_easy(source: &str) -> String {
    let mut sys = compile(source);
    vm::run(&mut sys);
    if sys.error.len() > 0 {
        return sys.error;
    }
    sys.output
}
