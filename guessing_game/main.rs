
// Imports the external crates so its functions can be used
use std::{cmp::Ordering, io};
use rand;
use colored::*;


// function to be called when the project is run
fn main() {
    println!("Guess The Number");
    
    // Generates a random number between 1 and 10 (inclusive)
    let secret_number = rand::random_range(1..=10);

    println!("random: {}", secret_number); // Debug print (can be removed in production)

    // loop till user win
    loop {
        println!("Input your Guess :");

        let mut guess = String::new(); // Creates a mutable String to store user input
        
        io::stdin()
            .read_line(&mut guess) // Reads a line of input from the user
            .expect("Failed to read line"); // If reading fails, show this error
        println!("Your Guess : {}", guess);

        // Trims the input and tries to parse it to an unsigned integer
        let guess : u32 = match guess.trim().parse(){
            // If parsing succeeds, use the number
            Ok(num) => num, 
            // If parsing fails, prompt again
            Err(_) => {
                println!("Please write a number"); 
                //go to the first line of the loop
                continue;
            }
        };

        // Compares the guess to the secret number and prints feedback
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()), // Guess is lower than secret number
            Ordering::Equal => {
                    println!("{}", "YOU WINNN".green()); // Correct guess
                    //if user win, break the loop and finish the game
                    break;
            },
            Ordering::Greater => println!("{}", "Too Big".red()), // Guess is higher than secret number
        }
    }

    // after loop is breaked, (you can add something here)
}