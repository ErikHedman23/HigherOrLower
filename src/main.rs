use rand::prelude::*;
use std::io;

fn main() {
    let rand_number = thread_rng().gen_range(1..101);
    let mut count = 0;
    println!(
        "You must guess a number between 1 and 100! 
        \nIf the number is wrong, you will have to guess until you get the right answer...
        \nYou only have 10 guesses...
        \nWhat is your guess: "
    );

    loop {
        let mut buffer = String::new();

        let _ = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input line.");
        let guessed_number = buffer
            .trim()
            .parse::<i32>()
            .expect("Failed to parse the number.");
        count += 1;
        if count == 10 {
            println!("You ran out of guesses!  Try again next time.  Thanks for playing!");
            break;
        } else if rand_number < guessed_number {
            println!("\nYour number was too high, try again!");
            println!("Current count of guesses: {}", count);
        } else if rand_number > guessed_number {
            println!("\nYour number was too low, try again!");
            println!("Current count of guesses: {}", count);
        } else {
            println!(
                "\nCongrats!  You found the number, and it is: {}",
                rand_number
            );
            break;
        }
    }
}
