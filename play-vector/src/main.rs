// build an example using vectors

fn main() {
    // create a vector
    let mut v = vec![1, 2, 3, 4, 5];

    // add a value to the vector
    v.push(6);

    // remove a value from the vector
    v.pop();

    // loop through the vector
    for i in &v {
        println!("{}", i);
    }

    // loop and mutate the vector
    for i in &mut v {
        *i += 50;
    }
}
