use std::io;


fn main() {
    // Show message
    println!("Guess the number!");
    println!("Please input your guess!");

    // Input...
    let mut guess = String::new();  // mut is mutable
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

}
