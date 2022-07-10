use clap::Parser;

mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(default_value_t = true, long, action)]
    print_display: bool,

    #[clap(long, action)]
    print_debug: bool,
}

fn main() {
    let args = Args::parse();
    if args.print_debug {
        println!("{:?}", args);
    }

    let mut state = cli::State {
        print_display: true,
        print_debug: args.print_debug,
    };
    cli::run(&mut state);
}
