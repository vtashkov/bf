//! Brainfuck interpreter
//!
//! # Examples
//!
//! This will output "Hello World!\n" in the output vector:
//!
//! ```
//! use std::io::Cursor;
//! use std::str;

//! use vtashkov_bf::Interpreter;

//! let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
//! let mut input = Cursor::new(vec![]);
//! let mut output = vec![];
//! let mut interpreter = Interpreter::new(&mut input, &mut output, 30000);
//! interpreter.execute(&source_code);
//! assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
//! ```
//!
use std::io::{Read, Write};

use crate::memory::Memory;

/// Brainfuck interpreter
pub struct Interpreter<'a, R, W>
where
    R: Read,
    W: Write,
{
    memory: Memory<u8>,
    input: &'a mut R,
    output: &'a mut W,
}

impl<'a, R, W> Interpreter<'a, R, W>
where
    R: Read,
    W: Write,
{
    /// Creates new interpreter with the given input stream, output stream and number of cells
    pub fn new(input: &'a mut R, output: &'a mut W, memory_size: usize) -> Interpreter<'a, R, W> {
        Interpreter {
            memory: Memory::new(memory_size),
            input,
            output,
        }
    }

    /// Executes a program
    pub fn execute(&mut self, source_code: &str) {
        self.memory.clear();
        let instructions = parse(&mut source_code.chars());
        self.execute_instructions(&instructions);
    }

    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::NextCell => self.memory.next(),
                Instruction::PreviousCell => self.memory.previous(),
                Instruction::IncrementData => self.memory.increment(),
                Instruction::DecrementData => self.memory.decrement(),
                Instruction::OutputData => {
                    let value = self.memory.read();
                    self.output.write(&[*value]).unwrap();
                }
                Instruction::InputData => {
                    let mut value = [0_u8];
                    if self.input.read_exact(&mut value).is_ok() {
                        self.memory.write(value[0]);
                    }
                }
                Instruction::Loop(loop_instructions) => {
                    while *self.memory.read() != 0 {
                        self.execute_instructions(loop_instructions)
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    NextCell,
    PreviousCell,
    IncrementData,
    DecrementData,
    OutputData,
    InputData,
    Loop(Vec<Instruction>),
}

fn parse(chars: &mut impl Iterator<Item = char>) -> Vec<Instruction> {
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
    use std::io::Cursor;

    use super::*;

    #[test]
    fn interpreter_can_be_created() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let _interpreter = Interpreter::new(&mut input, &mut output, 1);
    }

    #[test]
    fn interpreter_can_execute_code() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute("");
    }

    #[test]
    fn interpreter_ignores_non_instructions() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(" !\"#$%&'()*/0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz{|}~");
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, output)
    }

    #[test]
    fn interpreter_outputs_default_cell_value_of_zero() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".");
        assert_eq!(vec![0], output)
    }

    #[test]
    fn interpreter_inputs_and_outputs_value() {
        let mut input = Cursor::new(vec![1]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(",.");
        assert_eq!(vec![1], output)
    }

    #[test]
    fn interpreter_inputs_and_outputs_multiple_value() {
        let mut input = Cursor::new(vec![1, 2]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".,.,.");
        assert_eq!(vec![0, 1, 2], output)
    }

    #[test]
    fn interpreter_does_not_change_anything_if_no_input_value() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(",.");
        assert_eq!(vec![0], output)
    }

    #[test]
    fn interpreter_increments_cell() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute("+.");
        assert_eq!(vec![1], output)
    }

    #[test]
    fn interpreter_decrements_cell() {
        let mut input = Cursor::new(vec![2]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(",.-.");
        assert_eq!(vec![2, 1], output)
    }

    #[test]
    fn interpreter_increments_and_decrements_cell_multiple_times() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".+.+.+.-.-.-.");
        assert_eq!(vec![0, 1, 2, 3, 2, 1, 0], output)
    }

    #[test]
    fn interpreter_moves_to_next_cell() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute(".+.>.");
        assert_eq!(vec![0, 1, 0], output)
    }

    #[test]
    fn interpreter_moves_back_and_forth() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute(".+.>.+.+.<.-.>.");
        assert_eq!(vec![0, 1, 0, 1, 2, 1, 0, 2], output)
    }

    #[test]
    fn interpreter_goes_back_to_first_cell_after_reaching_the_end() {
        let mut input = Cursor::new(vec![1, 2, 3]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 3);
        interpreter.execute(",>,>,>.>.>.");
        assert_eq!(vec![1, 2, 3], output)
    }

    #[test]
    fn interpreter_goes_back_to_the_end_if_going_back_from_the_first_cell() {
        let mut input = Cursor::new(vec![1, 2, 3]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 3);
        interpreter.execute(",>,>,><.<.<.");
        assert_eq!(vec![3, 2, 1], output)
    }

    #[test]
    fn interpreter_skips_loop_if_current_cell_is_zero() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".[.].");
        assert_eq!(vec![0, 0], output)
    }

    #[test]
    fn interpreter_executes_loop_once_if_current_cell_is_non_zero() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".+[.-].");
        assert_eq!(vec![0, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_loop_twice_if_current_cell_is_two() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 1);
        interpreter.execute(".++[.-].");
        assert_eq!(vec![0, 2, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_embedded_loops() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[>++[.-].<.-].");
        assert_eq!(vec![2, 1, 0, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_two_embedded_loops() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 3);
        interpreter.execute("+[>+[.-]+[.-].<.-].");
        assert_eq!(vec![1, 1, 0, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_double_embedded_loops() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 3);
        interpreter.execute("+[->++[->+[.-]<]]");
        assert_eq!(vec![1, 1], output)
    }

    #[test]
    fn interpreter_executes_no_end_loop() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 3);
        interpreter.execute(".+.[-.");
        assert_eq!(vec![0, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_embedded_no_end_loop() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[>++[.-].<.-");
        assert_eq!(vec![2, 1, 0, 1], output)
    }

    #[test]
    fn interpreter_executes_no_embedded_end_loop() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[>++[.-.<.-].");
        assert_eq!(vec![2, 1, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_embedded_no_end_loops() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[>++[.-.<.-");
        assert_eq!(vec![2, 1, 1], output)
    }

    #[test]
    fn interpreter_executes_two_embedded_no_end_loop() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[>+[.-]+[.-].<.-].");
        assert_eq!(vec![1, 1, 0, 1, 0], output)
    }

    #[test]
    fn interpreter_executes_double_embedded_no_end_loops() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute("+[->++[->+[.-<]]");
        assert_eq!(vec![1, 1], output)
    }

    #[test]
    fn interpreter_executes_only_end_loop() {
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let mut interpreter = Interpreter::new(&mut input, &mut output, 2);
        interpreter.execute(".].");
        assert_eq!(vec![0], output)
    }
}
