use clap::Parser;
use rand::prelude::*;

/// A simple CLI to generate random numbers in a given range takes an x and y
/// value and generates a random number between them

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift")]
#[command(about = "Generates a Random Number between an X and Y Range", long_about = None)]
struct Opts {
    /// The lower bound of the range
    #[clap(short, long, default_value = "0")]
    x: i32,
    /// The upper bound of the range
    #[clap(short, long, default_value = "100")]
    y: i32,
}

/// Function that generates a random number between x and y
fn generate_random_number(x: i32, y: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x..y)
}

fn main() {
    let opts: Opts = Opts::parse();
    let random_number = generate_random_number(opts.x, opts.y);
    println!("Random number between {} and {} is {}", opts.x, opts.y, random_number);
}
