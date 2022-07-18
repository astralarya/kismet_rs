use clap::Parser;

mod cli;
use cli::PrintLevel;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(arg_enum, default_value_t = PrintLevel::Output, long, action)]
    print: PrintLevel,
}

fn main() {
    let args = Args::parse();
    if let PrintLevel::Debug = args.print {
        println!("{:?}", args);
    }

    let mut state = cli::State { print: args.print };
    cli::run(&mut state);
}
