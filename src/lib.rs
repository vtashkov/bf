mod interpreter;
mod memory;

use std::{
    fs,
    io::{self, Read, Write},
};

pub use interpreter::Interpreter;

pub fn run_cmd(
    args: Vec<String>,
    stdin: &mut impl Read,
    stdout: &mut impl Write,
) -> Result<(), String> {
    let input_file_path = parse_cmd_arguments(args)?;
    let source_code = read_file_contents(&input_file_path)?;
    let mut interpreter = Interpreter::new(stdin, stdout, 30000);
    interpreter.execute(&source_code);
    Ok(())
}

fn parse_cmd_arguments(args: Vec<String>) -> Result<String, String> {
    if args.len() < 2 {
        return Err(String::from("no input file"));
    }
    let input_file_path = args[1].clone();
    Ok(input_file_path)
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
        let args = vec![];
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let _ = run_cmd(args, &mut input, &mut output);
    }

    #[test]
    fn run_cmd_with_no_args_returns_error() {
        let args = vec![];
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_err());
    }

    #[test]
    fn run_cmd_with_less_than_two_args_returns_no_input_file() {
        let args = vec![];
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_err());
        assert_eq!("no input file", result.err().unwrap())
    }

    #[test]
    fn run_cmd_with_wrong_input_file_returns_no_such_file() {
        let invalid_file_name = "./examples/invalid.bf";
        let args = vec![
            String::from("bf"),
            String::from(invalid_file_name),
        ];
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_err());
        let expected_error_message = format!("no such file: '{}'", invalid_file_name);
        assert_eq!(expected_error_message, result.err().unwrap())
    }

    #[test]
    fn run_cmd_can_execute_hello_world() {
        let args = vec![
            String::from("bf"),
            String::from("./examples/hello_world.bf"),
        ];
        let mut input = Cursor::new(vec![]);
        let mut output = vec![];
        let result = run_cmd(args, &mut input, &mut output);
        assert!(result.is_ok());
        assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
    }
}
