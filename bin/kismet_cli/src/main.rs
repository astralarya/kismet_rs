use clap::Parser;

mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long, action)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let mut state = cli::State {
        verbose: args.verbose,
    };
    cli::run(&mut state);
}
