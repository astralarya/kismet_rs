use clap::ArgEnum;
use kismet_language::parse;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct State {
    pub print: PrintLevel,
}

#[derive(Clone, Debug, ArgEnum)]
pub enum PrintLevel {
    None,
    Output,
    Debug,
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
                        Ok(x) => match state.print {
                            PrintLevel::Debug | PrintLevel::Output => println!("{:#?}", x),
                            // PrintLevel::Debug => println!("{:#?}\n{}", x, x),
                            // PrintLevel::Output => println!("{}", x),
                            PrintLevel::None => (),
                        },
                        Err(e) => eprintln!("{}", e),
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
