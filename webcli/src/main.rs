extern crate wikipedia;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift")]
#[command(about = "Returns wikipedia page", long_about = None)]
struct Opts {
    /// The title of the page
    #[clap(short, long, default_value = "0")]
    title: String,
}   



/// build a wikipedia page from a title
fn wiki () {
    let opts: Opts = Opts::parse();
    /// print option
    println!("title: {}", opts.title);
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title("Club Atletico River Plate".to_owned());
    let content = page.get_content().unwrap();
    println!("{}", content);
}


fn main() {
    wiki();
}
