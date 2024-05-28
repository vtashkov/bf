#[derive(PartialEq, Debug)]
pub enum Token {
    NextCell,
    PreviousCell,
    IncrementData,
    DecrementData,
    OutputData,
    InputData,
    BeginLoop,
    EndLoop,
}

pub fn lex(source_code: &str) -> Vec<Token> {
    source_code
        .chars()
        .filter_map(|ch| match ch {
            '>' => Some(Token::NextCell),
            '<' => Some(Token::PreviousCell),
            '+' => Some(Token::IncrementData),
            '-' => Some(Token::DecrementData),
            '.' => Some(Token::OutputData),
            ',' => Some(Token::InputData),
            '[' => Some(Token::BeginLoop),
            ']' => Some(Token::EndLoop),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_next_cell_token() {
        let tokens = lex(">");
        assert_eq!(vec![Token::NextCell], tokens);
    }

    #[test]
    fn lex_previous_cell_token() {
        let tokens = lex("<");
        assert_eq!(vec![Token::PreviousCell], tokens);
    }

    #[test]
    fn lex_increment_data_token() {
        let tokens = lex("+");
        assert_eq!(vec![Token::IncrementData], tokens);
    }

    #[test]
    fn lex_decrement_data_token() {
        let tokens = lex("-");
        assert_eq!(vec![Token::DecrementData], tokens);
    }

    #[test]
    fn lex_output_data_token() {
        let tokens = lex(".");
        assert_eq!(vec![Token::OutputData], tokens);
    }

    #[test]
    fn lex_input_data_token() {
        let tokens = lex(",");
        assert_eq!(vec![Token::InputData], tokens);
    }

    #[test]
    fn lex_begin_loop_token() {
        let tokens = lex("[");
        assert_eq!(vec![Token::BeginLoop], tokens);
    }

    #[test]
    fn lex_end_loop_token() {
        let tokens = lex("]");
        assert_eq!(vec![Token::EndLoop], tokens);
    }

    #[test]
    fn lex_ignores_other_symbols() {
        let tokens = lex(" !\"#$%&'()*/0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz{|}~");
        assert_eq!(Vec::<Token>::new(), tokens);
    }

    #[test]
    fn lex_mixed_symbols() {
        let tokens = lex("> < + - . , [ ] ");
        assert_eq!(
            vec![
                Token::NextCell,
                Token::PreviousCell,
                Token::IncrementData,
                Token::DecrementData,
                Token::OutputData,
                Token::InputData,
                Token::BeginLoop,
                Token::EndLoop
            ],
            tokens
        );
    }
}
