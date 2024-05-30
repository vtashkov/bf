use std::{env, fs, io::{self, stdin, stdout}, process};

fn main() {
    let input_file_path = parse_cmd_arguments().unwrap_or_else(error_and_exit());
    let source_code = read_file_contents(&input_file_path).unwrap_or_else(error_and_exit());
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut interpreter = bf::Interpreter::new(&mut stdin, &mut stdout, 30000);
    interpreter.execute(&source_code);
}

fn parse_cmd_arguments() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
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

fn error_and_exit() -> impl FnOnce(String) -> String {
    |err| {
        eprintln!("bf: error: {err}");
        process::exit(1);
    }
}
