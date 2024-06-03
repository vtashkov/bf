//! Brainfuck interpreter
//!
//! # Examples
//!
//! This will output "Hello World!\n" in the output vector:
//!
//! ```
//! use std::io::Cursor;
//! use std::str;
//! 
//! use vtashkov_bf::Interpreter;
//! use vtashkov_bf::Program;
//! 
//! let mut input = Cursor::new(vec![]);
//! let mut output = vec![];
//! let mut interpreter = Interpreter::new(&mut input, &mut output, 30000);
//! let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
//! let program = Program::parse(source_code);
//! interpreter.execute(program);
//! assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
//! ```

/// The Brainfuck interpreter
mod interpreter;

/// Memory cells for the interpreter (memory tape)
mod memory;
mod program;

use std::{
    fs,
    io::{self, Read, Write},
};

use clap::Parser;

// re-exports
pub use interpreter::Interpreter;
pub use program::Program;

/// Command-line arguments for the interpreter
/// input_file - the path to the file to be interpreted
/// memory_size - the number of the cells in the memory, defaults to 30 000
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the file to be interpreted
    #[arg()]
    input_file: String,

    /// Number of the cells in the memory, defaults to 30 000
    #[arg(short, long, default_value_t = 30000)]
    memory_size: usize,
}

/// Runs the interpreter using the arguments passed - file to read the source from and memory size
pub fn run(args: Args, input: &mut impl Read, output: &mut impl Write) -> Result<(), String> {
    let source_code = read_file_contents(&args.input_file)?;
    let mut interpreter = Interpreter::new(input, output, args.memory_size);
    let program = Program::parse(&source_code);
    interpreter.execute(program);
    Ok(())
}

fn read_file_contents(input_file_path: &str) -> Result<String, String> {
    fs::read_to_string(input_file_path).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => format!("no such file: '{input_file_path}'"),
        _ => error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use io::Cursor;
    use std::str;

    use super::*;

    #[test]
    fn run_can_be_invoked() {
        let args = Args {
            input_file: String::from(""),
            memory_size: 1,
        };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let _ = run(args, &mut input, &mut output);
    }

    #[test]
    fn run_with_wrong_input_file_returns_no_such_file() {
        let invalid_file_name = "./examples/invalid.bf";
        let args = Args {
            input_file: String::from(invalid_file_name),
            memory_size: 1,
        };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run(args, &mut input, &mut output);
        assert!(result.is_err());
        let expected_error_message = format!("no such file: '{}'", invalid_file_name);
        assert_eq!(expected_error_message, result.err().unwrap())
    }

    #[test]
    fn run_can_execute_hello_world() {
        let args = Args {
            input_file: String::from("./examples/hello_world.bf"),
            memory_size: 30000,
        };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run(args, &mut input, &mut output);
        assert!(result.is_ok());
        assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
    }
}
