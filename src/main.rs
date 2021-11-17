extern crate rand;

use std::io;
use rand::Rng;


fn main() {
    // Show message
    println!("Guess the number!");

    // Create guess number
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_num);

    // Input...
    println!("Please input your guess!");
    let mut guess = String::new();  // mut is mutable
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

}
