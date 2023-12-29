// program that take user input and outputs
use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter your name:");

    // Create a mutable string to store the user's input
    let mut input = String::new();

    // Read a line of text from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Print the user's input
    println!("Hello, {}!", input);
}




