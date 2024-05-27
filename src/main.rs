use std::{env, fs, io::ErrorKind, process};

fn main() {
    
    let file_path = parse_cmd_arguments().unwrap_or_else(error_and_abort());

    let contents = read_file(&file_path).unwrap_or_else(error_and_abort());

    println!("{contents}");
}

fn parse_cmd_arguments() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(String::from("no input file"));
    }

    let file_path = args[1].clone();

    Ok(file_path)
}

fn read_file(file_path: &str) -> Result<String, String> {
    let contents_result = fs::read_to_string(file_path);

    let contents = match contents_result {
        Ok(contents) => contents,
        Err(error) => { 
            match error.kind() {
                ErrorKind::NotFound => return Err(format!("no such file: '{file_path}'")),
                _ => return Err(error.to_string())
            }
        }
    };

    Ok(contents)
}

fn error_and_abort() -> impl FnOnce(String) -> String {
    |err| {
        eprintln!("bf: error: {err}");
        process::exit(1);
    }
}