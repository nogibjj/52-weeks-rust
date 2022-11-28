# 52-weeks-rust
Trying out Rust

## How to Install

* [Install Steps](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Pro-tip, run `rustup update` to update your installation.
* Check version do this: `rustc --version`

## How the heck do I compile?

* `rustc myfunc.rs` in this case, the `<yourfile>.rs` is what is compiled.
* Next you run it:  ./myfunc.rs

## How do I use Cargo?

* Do you have it? `cargo --version`?
* `cargo new hello_cargo && cd hello_cargo`

## Ep 14: Build a rust cli that calls out to the web somehow


## Ep 13: Use popular rust crate + cli

Goal:  Build a cli clap tool that generates random numbers in a range

* [clap docs](https://docs.rs/clap/latest/clap/)

To run it: `cargo run -- -x 1 -y 10`

To run help:

`cargo run -- --help`
The output should be:
```bash
Generates a Random Number between an X and Y Range

Usage: randocli [OPTIONS]

Options:
  -x, --x <X>    The lower bound of the range [default: 0]
  -y, --y <Y>    The upper bound of the range [default: 100]
  -h, --help     Print help information
  -V, --version  Print version information
```


## Episode 12:  Finished Setup and Using Vectors also try rust-cli book examples
* Added setup script for configuring codespaces:  https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html
* Shoutout the [rust-cli book](https://github.com/rust-cli/book)
`cargo new grrs`

To run this tool do the following and you will see this output:
`cargo run -- fancy foo.txt `

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/grrs fancy foo.txt`
Searching for Pattern: fancy
Found Pattern: fancy
fancy
```


## Episode 11: Fix Codespaces Rust

* Environment seems totally fubar, figure out how to make one that isn't.
* Also figure out how to not checkin debug files

## Episode 10:  Collections

* [collections and vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)

### Note:  Run into an weird issue and tried

`cargo install cargo --force`


## Episode 9:  Working with modules and packages

* [Cover modules and packages](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

* Create is smallest container (contain main)
* Library crates don’t have a main function, and they don’t compile to an executable. 
* A package is a bundle of one or more crates that provides a set of functionality
* Cargo.toml file that describes how to build those crates

`cargo new backyard`

TBD:  Still not working...dive into this more next time:  https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html


## Episode 8: Figuring how cli tools work with Rust

* The target binary is in target/debug.  But, should I run this for day to day or should I use Cargo?
* change the way you pass things in:  https://stackoverflow.com/questions/73364389/i-cant-seem-to-get-my-clap-parser-to-take-in-a-vector-of-string-and-use-flags


The way to run a cli via cargo is with these fancy `--` and then don't say name of tool
```bash
cargo run -- --name "bob"
```


## Episode 7


## Episode 6

Fixed random choices and started [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)


## Episode 5

We fixed it!

Fix this CODE!!!

```rust
// Looping fun
// Thank you:  https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
use rand::Rng;

// Broken code fix:  learn to convert to iterator
fn main() {
    //let a = [10, 20, 30, 40, 50];
    let mut rng = rand::thread_rng();
    for element in rng {
        println!("the value is: {element}");
    }
}
```

## Episode 3

Make a function:

* `cargo new hello_func && cd hello_func` (put some code in src/main like this)

```rust
//Function 1
fn main() {
    println!("Hello, world!");

    another_function();
}

//Function 2
fn another_function() {
    println!("Another function.");
}
```

* `cargo build`

## Episode 2
## Trying to learn Rust

Quick example:
```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

* [learn by example guide](https://doc.rust-lang.org/stable/rust-by-example/)

## References

* [Codespaces Examples Rust](https://github.com/codespaces-examples/rust)
* [updating cargo and rust](https://stackoverflow.com/questions/37928591/is-there-a-command-to-update-cargo-to-the-latest-official-release)