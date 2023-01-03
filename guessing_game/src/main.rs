// import types and crates
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// define  main function
fn main() {
    
    // generate random number using rand crate
    let secret_number = rand::thread_rng().gen_range(0..=100);

    // define a mutable variable to store the number of guesses the user has made
    let mut _number_of_guesses = 0;

    // loop dialogue until user guesses the correct number
    loop {

        // print guess the number inclusive of 0 and 100
        println!("Guess a number inclusive of 0 and 100...");
        // create mutable variable to store user input as string
        let mut guess = String::new();
        // read user input and store in guess variable
        io::stdin()
            // call read_line method on standard input referenced guess
            .read_line(&mut guess)
            // handle error with expect method, if error occurs, program will crash and display message
            .expect("Failed to read line");
            // check if user input is a number, trim whitespace, and parse to u32, if not a number, continue the loop until a number is entered
        let guess: u32 = match guess.trim().parse() {
            // Result Ok if number, Err if not a number, and add 1 to number of guesses
            Ok(num) => {
                _number_of_guesses += 1;
                num
            },
            // Error, print that the input must be a number, add 1 to the number of guesses, and continue loop if not a number

            Err(_) => {
                println!("Please input a number!");
                _number_of_guesses += 1;
                continue;
            }    
        };

        // print user input
        println!("You guessed: {guess}");
        // compare guess to secret number and match the result, then print the result of that match, if guess is equal to secret number, break the loop
        match guess.cmp(&secret_number) {
            // define Less match case
            Ordering::Less => println!("Too small!"),
            // define Greater match case
            Ordering::Greater => println!("Too big!"),
            // define Equal match case
            Ordering::Equal => {
                println!("You win! You guessed the number in {} guesses!", _number_of_guesses);
                // break the loop and end the program
                break;
            }
        }
    }
}
