use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

//invoke the program cargo run -- some-pattern some-file.txt
fn main() {
    let args = Cli::parse();
    //print out the arguments passed into the command line
    println!("CLI Arguments passed in: {:?}", args);

}