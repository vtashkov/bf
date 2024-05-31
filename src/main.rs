use std::{
    io::{stdin, stdout},
    process,
};

use bf::run_cmd;
use clap::Parser;

fn main() {
    run_cmd(bf::Args::parse(), &mut stdin(), &mut stdout()).unwrap_or_else(|err| {
        eprintln!("bf: error: {err}");
        process::exit(1);
    });
}
