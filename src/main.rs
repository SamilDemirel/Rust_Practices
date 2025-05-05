use std::{cmp::Ordering, io};
use rand;
use colored::*;



fn main() {
    println!("Guess The Number");
    
    let secret_number = rand::random_range(1..=10);

    println!("random: {}", secret_number);

    loop {
        println!("Input your Guess :");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your Guess : {}", guess);

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please write a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Equal => {
                    println!("{}","YOU WINNN".green());
                    break;
            },
            Ordering::Greater => println!("{}","Too Big".red()),
            
        }
    }

}
