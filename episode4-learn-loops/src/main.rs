// Thanks SO:  https://stackoverflow.com/questions/29201370/error-expected-item-found-let
// This is the crate: https://crates.io/search?q=random_choice

extern crate random_choice;
use self::random_choice::random_choice;

fn main() {
let mut samples = vec!["hi", "this", "is", "a", "test!"];
let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

let number_choices = 2;
let choices = random_choice().random_choice_f64(&samples, &weights, number_choices);

for choice in choices {
    print!("{}, ", choice);
}
 }