// Rng trait defines methods that random number generators implement
use rand::Rng;
// Ordering is an Enum type has the variants Less, Greater, and Equal.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // We might delete this print statement in the real game.
    println!("The secret number is: {secret_number}");

    loop {
        println!("--------- Start Guessing ----------");
        println!("Please input your guess!");
        // new(): an associated function that works wit many types
        // guess is a growable, UTF-8 encoded bit of text
        let mut guess = String::new();

        // handle input: tell read_line what string to store input from user
        // read_line() returns a Result value
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read the line.");

        // let my_guess: u32 = guess.parse().expect("Not a number!");

        // Parse the string input into an integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input integer please!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
