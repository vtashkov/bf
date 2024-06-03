use std::{
    io::{stdin, stdout},
    process,
};

use clap::Parser;
use vtashkov_bf::{run, Args};

fn main() {
    run(Args::parse(), &mut stdin(), &mut stdout()).unwrap_or_else(|err| {
        eprintln!("vtashkov-bf: error: {err}");
        process::exit(1);
    });
}
