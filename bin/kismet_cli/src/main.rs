use std::collections::HashSet;

use clap::{ArgEnum, Parser};

mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(multiple = true, long, action, help = "Default: [output, error]\n ")]
    print: Vec<Print>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ArgEnum)]
pub enum Print {
    Debug,
    Ast,
    Loopback,
    Output,
    Error,
}

fn main() {
    let args = Args::parse();
    let mut print = HashSet::new();
    print.insert(Print::Output);
    print.insert(Print::Error);
    args.print.clone().into_iter().for_each(|x| {
        print.insert(x);
    });

    if print.contains(&Print::Debug) {
        println!("{:?}", args);
    }

    let mut state = cli::State { print };
    cli::run(&mut state);
}
