use std::{env, io::{stdin, stdout}, process};

use bf::run_cmd;

fn main() {
    run_cmd(env::args().collect(), &mut stdin(), &mut stdout()).unwrap_or_else(error_and_exit());
}

fn error_and_exit() -> impl FnOnce(String) {
    |err| {
        eprintln!("bf: error: {err}");
        process::exit(1);
    }
}