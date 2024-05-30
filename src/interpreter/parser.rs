use super::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    NextCell,
    PreviousCell,
    IncrementData,
    DecrementData,
    OutputData,
    InputData,
    Loop(Vec<Instruction>),
}

pub fn parse(tokens: &mut impl Iterator<Item = Token>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    while let Some(token) = tokens.next() {
        instructions.push(match token {
            Token::NextCell => Instruction::NextCell,
            Token::PreviousCell => Instruction::PreviousCell,
            Token::IncrementData => Instruction::IncrementData,
            Token::DecrementData => Instruction::DecrementData,
            Token::OutputData => Instruction::OutputData,
            Token::InputData => Instruction::InputData,
            Token::BeginLoop => Instruction::Loop(parse(tokens)),
            Token::EndLoop => break,
        })
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(s: &str) -> Vec<Token> {
        s.chars()
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

    #[test]
    fn parse_next_cell_instruction() {
        let tokens = tokens(">");
        let expected = vec![Instruction::NextCell];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_previous_cell_instruction() {
        let tokens = tokens("<");
        let expected = vec![Instruction::PreviousCell];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_increment_data_instruction() {
        let tokens = tokens("+");
        let expected = vec![Instruction::IncrementData];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_decrement_data_instruction() {
        let tokens = tokens("-");
        let expected = vec![Instruction::DecrementData];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_output_data_instruction() {
        let tokens = tokens(".");
        let expected = vec![Instruction::OutputData];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_input_data_instruction() {
        let tokens = tokens(",");
        let expected = vec![Instruction::InputData];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_empty_loop_instruction() {
        let tokens = tokens("[]");
        let expected = vec![Instruction::Loop(vec![])];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_simple_one_instruction_loop_instruction() {
        let tokens = tokens("[+]");
        let expected = vec![Instruction::Loop(vec![Instruction::IncrementData])];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_simple_multiple_instructions_loop_instruction() {
        let tokens = tokens("[+-.,]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::DecrementData,
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_inbetween_simple_loop_instruction() {
        let tokens = tokens(".[+]-");
        let loop_instructions = vec![Instruction::IncrementData];
        let expected = vec![
            Instruction::OutputData,
            Instruction::Loop(loop_instructions),
            Instruction::DecrementData,
        ];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_loop_instruction() {
        let tokens = tokens("[+[-].,]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_two_embedded_loop_instruction() {
        let tokens = tokens("[+[-][.],]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::Loop(vec![Instruction::OutputData]),
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_double_embedded_loop_instruction() {
        let tokens = tokens("[+[-[.]],]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::Loop(vec![Instruction::OutputData]),
            ]),
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_no_end_loop_instruction() {
        let tokens = tokens("[+");
        let expected = vec![Instruction::Loop(vec![Instruction::IncrementData])];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_inbetween_no_end_loop_instruction() {
        let tokens = tokens(".[+-");
        let loop_instructions = vec![Instruction::IncrementData, Instruction::DecrementData];
        let expected = vec![
            Instruction::OutputData,
            Instruction::Loop(loop_instructions),
        ];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_no_end_loop_instruction() {
        let tokens = tokens("[+[-].,");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_no_embedded_end_loop_instruction() {
        let tokens = tokens("[+[-.,]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::OutputData,
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_no_end_loops_instruction() {
        let tokens = tokens("[+[-.,");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::OutputData,
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_two_embedded_no_end_loop_instruction() {
        let tokens = tokens("[+[-][.,]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::Loop(vec![Instruction::OutputData, Instruction::InputData]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_double_embedded_end_loop_instruction() {
        let tokens = tokens("[+[-[.],]");
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::Loop(vec![Instruction::OutputData]),
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_only_end_loop_instruction() {
        let tokens = tokens(".+]-");
        let expected = vec![Instruction::OutputData, Instruction::IncrementData];
        let instructions = parse(&mut tokens.into_iter());
        assert_eq!(expected, instructions);
    }
}
