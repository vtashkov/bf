mod interpreter;
mod memory;

use std::{
    fs,
    io::{self, Read, Write},
};

use clap::Parser;
pub use interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg()]
    input_file: String,

    #[arg(short, long, default_value_t = 30000)]
    memory_size: usize,
}

pub fn run_cmd(args: Args, stdin: &mut impl Read, stdout: &mut impl Write) -> Result<(), String> {
    let source_code = read_file_contents(&args.input_file)?;
    let mut interpreter = Interpreter::new(stdin, stdout, args.memory_size);
    interpreter.execute(&source_code);
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
    fn run_cmd_can_be_invoked() {
        let args = Args { input_file: String::from(""), memory_size: 1 };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let _ = run_cmd(args, &mut input, &mut output);
    }

    #[test]
    fn run_cmd_with_wrong_input_file_returns_no_such_file() {
        let invalid_file_name = "./examples/invalid.bf";
        let args = Args { input_file: String::from(invalid_file_name), memory_size: 1 };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_err());
        let expected_error_message = format!("no such file: '{}'", invalid_file_name);
        assert_eq!(expected_error_message, result.err().unwrap())
    }

    #[test]
    fn run_cmd_can_execute_hello_world() {
        let args = Args { input_file: String::from("./examples/hello_world.bf"), memory_size: 30000 };
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_ok());
        assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
    }
}
