pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { }
    }

    pub fn execute(&self, source_code: &str) {
        let _tokens = lex(source_code);
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Token {
    NextCell,
    PreviousCell,
    IncrementData,
    DecrementData,
    OutputData,
    InputData,
    BeginLoop,
    EndLoop,
}

fn lex(source_code: &str) -> Vec<Token> {
    source_code.chars().filter_map(|ch| {
        match ch {
            '>' => Some(Token::NextCell),
            '<' => Some(Token::PreviousCell),
            '+' => Some(Token::IncrementData),
            '-' => Some(Token::DecrementData),
            '.' => Some(Token::OutputData),
            ',' => Some(Token::InputData),
            '[' => Some(Token::BeginLoop),
            ']' => Some(Token::EndLoop),
            _ => None,
        }
    }).collect()
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

    #[test]
    fn lexer_can_lex_code() {
        lex("");
    }

    #[test]
    fn lexer_can_lex_next_cell_token() {
        let tokens = lex(">");
        assert_eq!(vec![Token::NextCell], tokens);
    }

    #[test]
    fn lexer_can_lex_previous_cell_token() {
        let tokens = lex("<");
        assert_eq!(vec![Token::PreviousCell], tokens);
    }

    #[test]
    fn lexer_can_lex_increment_data_token() {
        let tokens = lex("+");
        assert_eq!(vec![Token::IncrementData], tokens);
    }

    #[test]
    fn lexer_can_lex_decrement_data_token() {
        let tokens = lex("-");
        assert_eq!(vec![Token::DecrementData], tokens);
    }

    #[test]
    fn lexer_can_lex_output_data_token() {
        let tokens = lex(".");
        assert_eq!(vec![Token::OutputData], tokens);
    }

    #[test]
    fn lexer_can_lex_input_data_token() {
        let tokens = lex(",");
        assert_eq!(vec![Token::InputData], tokens);
    }

    #[test]
    fn lexer_can_lex_begin_loop_token() {
        let tokens = lex("[");
        assert_eq!(vec![Token::BeginLoop], tokens);
    }

    #[test]
    fn lexer_can_lex_end_loop_token() {
        let tokens = lex("]");
        assert_eq!(vec![Token::EndLoop], tokens);
    }

    #[test]
    fn lexer_ignores_other_symbols() {
        let tokens = lex(" !\"#$%&'()*/0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz{|}~");
        assert_eq!(Vec::<Token>::new(), tokens);
    }
}
