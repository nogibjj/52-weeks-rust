use clap::Parser;

/// Build a phrase by repeating a word a number of times
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Word to repeat
    #[clap(short, long, value_parser)]
    word: String,

    /// Number of times to repeat
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    for _ in 0..args.count {
        print!("{} ", args.word)
    }
}