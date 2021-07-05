#![feature(with_options)]
#![feature(test)]

use std::env;
use std::fs;

use clap::{App, Arg};
use itertools::Itertools;

use crate::lexer::new as new_lexer;

mod ast;
mod interp;
mod lexer;
mod parser;
mod token;

fn split_args() -> (Vec<String>, Vec<String>) {
    let args = env::args().collect_vec();

    if let Some(i) = args.iter().position(|arg| arg == "--") {
        (args[..i].to_vec(), args[i + 1..].to_vec())
    } else {
        (args, vec![])
    }
}

fn main() {
    let (koi_args, script_args) = split_args();

    let matches = App::new("Koi")
        .version("0.1.0")
        .author("original work by Elia Perantoni <perantonielia0@gmail.com>")
        .arg(Arg::with_name("command").help("The command to invoke"))
        .get_matches_from(koi_args);

    let source = fs::read("Koifile").expect("Could not find Koifile");
    let lexer = new_lexer(String::from_utf8(source).expect("Koifile is not in valid utf-8"));

    let mut parser = parser::Parser::new(lexer);
    let prog = parser.parse();

    let mut interpreter = interp::Interpreter::new();
    interpreter.set_args(script_args);
    // interpreter.set_root(); // TODO: set the root to the path of the Koifile
    interpreter.run(prog);

    if let Some(f) = matches.value_of("command") {
        use ast::{Expr, Stmt};

        interpreter.run(vec![Stmt::Expr(Expr::Call {
            func: Box::new(Expr::Get(f.to_string())),
            args: vec![],
        })]);
    }
}
