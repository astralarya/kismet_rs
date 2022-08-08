use std::collections::HashSet;

use kismet::compile;
use kismet::hlir::{Exec, SymbolTable, Value};
use kismet::parse;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::Print;

pub struct State {
    pub print: HashSet<Print>,
}

pub fn run(state: &mut State) {
    println!(
        "\
        Hello, I am Kismet <3\n\
        Input a roll and press ENTER.\n\
        Exit with 'exit' or CTRL-D.\
        "
    );

    let mut rl = Editor::<()>::new();
    let mut i = SymbolTable::<Value>::default();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "exit" {
                    println!("Goodbye <3");
                    break;
                } else {
                    match parse(&line) {
                        Ok(x) => {
                            if state.print.contains(&Print::Ast) {
                                println!("{:#?}", x)
                            }
                            if state.print.contains(&Print::Loopback) {
                                println!("{}", x)
                            }
                            match compile(x) {
                                Ok(x) => match x.exec(i.clone()) {
                                    Ok((i_, val)) => {
                                        i = i_;
                                        if state.print.contains(&Print::Output) {
                                            println!("{}", val)
                                        }
                                    }
                                    Err(x) => {
                                        if state.print.contains(&Print::Error) {
                                            println!("Runtime Error: {:#?}", x)
                                        }
                                    }
                                },
                                Err(x) => {
                                    if state.print.contains(&Print::Error) {
                                        println!("Compile Error: {:#?}", x)
                                    }
                                }
                            }
                        }
                        Err(x) => {
                            if state.print.contains(&Print::Error) {
                                println!("Parse Error: {:#?}", x)
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
}
