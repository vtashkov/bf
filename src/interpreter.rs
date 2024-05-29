mod lexer;
mod parser;

use lexer::lex;
use parser::parse;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn execute(&self, source_code: &str) {
        let tokens = lex(source_code);
        let _instructions = parse(&mut tokens.into_iter());
        dbg!(_instructions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpreter_can_be_created() {
        let _interpreter = Interpreter::new();
    }

    #[test]
    fn interpreter_can_execute_code() {
        let interpreter = Interpreter::new();
        interpreter.execute("");
    }
}
