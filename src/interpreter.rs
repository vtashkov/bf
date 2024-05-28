mod lexer;

use lexer::lex;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn execute(&self, source_code: &str) {
        let _tokens = lex(source_code);
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
