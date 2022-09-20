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
