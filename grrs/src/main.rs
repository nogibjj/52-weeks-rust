#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift.")]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("Searching for Pattern: {}", args.pattern);
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    //print found pattern
    if content.contains(&args.pattern) {
        println!("Found Pattern: {}", args.pattern);
    } else {
        println!("Pattern Not Found: {}", args.pattern);
    }
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}