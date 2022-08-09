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