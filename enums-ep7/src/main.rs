//https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
//https://docs.github.com/en/codespaces/codespaces-reference/using-github-copilot-in-github-codespaces

fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
}
