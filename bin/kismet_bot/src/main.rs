use std::io;
use std::io::Write;

fn main() {
    println!(
        "\
        Hello, I am Kismet <3\n\
        Input a roll and press ENTER.\n\
        Exit with 'exit' or CTRL-D.\n\
        "
    );

    loop {
        print!("> ");
        if let Err(error) = io::stdout().flush() {
            panic!("{}", error)
        }

        let mut raw_input = String::new();
        io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read line");
        let input = raw_input.trim();
        if input == "exit" {
            println!("Goodbye <3");
            break;
        } else {
            println!("{}", input)
        }
    }
}
