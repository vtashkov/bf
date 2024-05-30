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

pub fn parse(chars: &mut impl Iterator<Item = char>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    while let Some(char) = chars.next() {
        instructions.push(match char {
            '>' => Instruction::NextCell,
            '<' => Instruction::PreviousCell,
            '+' => Instruction::IncrementData,
            '-' => Instruction::DecrementData,
            '.' => Instruction::OutputData,
            ',' => Instruction::InputData,
            '[' => Instruction::Loop(parse(chars)),
            ']' => break,
            _ => continue,
        });
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ignores_non_instructions() {
        let source_code = " !\"#$%&'()*/0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let expected: Vec<Instruction> = vec![];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_next_cell_instruction() {
        let source_code = ">";
        let expected = vec![Instruction::NextCell];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_previous_cell_instruction() {
        let source_code = "<";
        let expected = vec![Instruction::PreviousCell];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_increment_data_instruction() {
        let source_code = "+";
        let expected = vec![Instruction::IncrementData];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_decrement_data_instruction() {
        let source_code = "-";
        let expected = vec![Instruction::DecrementData];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_output_data_instruction() {
        let source_code = ".";
        let expected = vec![Instruction::OutputData];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_input_data_instruction() {
        let source_code = ",";
        let expected = vec![Instruction::InputData];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_empty_loop_instruction() {
        let source_code = "[]";
        let expected = vec![Instruction::Loop(vec![])];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_mixed_instruction() {
        let source_code = " > < + - . , [ ] ";
        let expected = vec![
            Instruction::NextCell,
            Instruction::PreviousCell,
            Instruction::IncrementData,
            Instruction::DecrementData,
            Instruction::OutputData,
            Instruction::InputData,
            Instruction::Loop(vec![]),
        ];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_simple_one_instruction_loop_instruction() {
        let source_code = "[+]";
        let expected = vec![Instruction::Loop(vec![Instruction::IncrementData])];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_simple_multiple_instructions_loop_instruction() {
        let source_code = "[+-.,]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::DecrementData,
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_inbetween_simple_loop_instruction() {
        let source_code = ".[+]-";
        let loop_instructions = vec![Instruction::IncrementData];
        let expected = vec![
            Instruction::OutputData,
            Instruction::Loop(loop_instructions),
            Instruction::DecrementData,
        ];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_loop_instruction() {
        let source_code = "[+[-].,]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_two_embedded_loop_instruction() {
        let source_code = "[+[-][.],]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::Loop(vec![Instruction::OutputData]),
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_double_embedded_loop_instruction() {
        let source_code = "[+[-[.]],]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::Loop(vec![Instruction::OutputData]),
            ]),
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_no_end_loop_instruction() {
        let source_code = "[+";
        let expected = vec![Instruction::Loop(vec![Instruction::IncrementData])];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_inbetween_no_end_loop_instruction() {
        let source_code = ".[+-";
        let loop_instructions = vec![Instruction::IncrementData, Instruction::DecrementData];
        let expected = vec![
            Instruction::OutputData,
            Instruction::Loop(loop_instructions),
        ];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_no_end_loop_instruction() {
        let source_code = "[+[-].,";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::OutputData,
            Instruction::InputData,
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_no_embedded_end_loop_instruction() {
        let source_code = "[+[-.,]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::OutputData,
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_embedded_no_end_loops_instruction() {
        let source_code = "[+[-.,";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::OutputData,
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_two_embedded_no_end_loop_instruction() {
        let source_code = "[+[-][.,]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![Instruction::DecrementData]),
            Instruction::Loop(vec![Instruction::OutputData, Instruction::InputData]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_double_embedded_end_loop_instruction() {
        let source_code = "[+[-[.],]";
        let loop_instructions = vec![
            Instruction::IncrementData,
            Instruction::Loop(vec![
                Instruction::DecrementData,
                Instruction::Loop(vec![Instruction::OutputData]),
                Instruction::InputData,
            ]),
        ];
        let expected = vec![Instruction::Loop(loop_instructions)];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn parse_only_end_loop_instruction() {
        let source_code = ".+]-";
        let expected = vec![Instruction::OutputData, Instruction::IncrementData];
        let instructions = parse(&mut source_code.chars());
        assert_eq!(expected, instructions);
    }
}
