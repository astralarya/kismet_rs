use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn run() {
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
                    println!("{}", line);
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
