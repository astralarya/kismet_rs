use std::collections::HashSet;

use kismet::exec;
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
                            match exec(x) {
                                Ok(x) => {
                                    if state.print.contains(&Print::Output) {
                                        println!("{}", x)
                                    }
                                }
                                Err(x) => {
                                    if state.print.contains(&Print::Error) {
                                        println!("ERROR: {:?}", x)
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("{:#?}", e),
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
