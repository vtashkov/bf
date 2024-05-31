use std::{
    io::{stdin, stdout},
    process,
};

use clap::Parser;
use vtashkov_bf::{run_cmd, Args};

fn main() {
    run_cmd(Args::parse(), &mut stdin(), &mut stdout()).unwrap_or_else(|err| {
        eprintln!("bf: error: {err}");
        process::exit(1);
    });
}
