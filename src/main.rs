extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    // Show message
    println!("Guess the number!");

    // Create target number
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_num);

    loop {
        // Input...
        println!("Please input your guess number(1-101)!");
        let mut guess = String::new();  // mut is mutable
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // Check validate
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num
            , Err(_) => {
                continue;
            }
        };
        
        println!("You guessed: {}", guess);

        // Input number compare to target number
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!")
            , Ordering::Greater => println!("Too large!")
            , Ordering::Equal => {
                println!("Match!");
                break;
            }
        }
    }
}
