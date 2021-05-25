#![allow(non_snake_case)]

use rand::Rng;


// simple function that return a random number between 0 and 100
fn get_random() -> i32 {
    let number = rand::thread_rng().gen_range(0..100);
    number
}

fn main() {
    println!("Hello, world!");

    // storing the random number in a variable for later use
    let number: i32 = get_random();

    // variable to store if the user found the correct number
    let mut correct: bool = false;
    // variable to store the number of guesses it took 
    let mut guesses: i8 = 0;

    // while the user is guessing (correct is false)
    while !correct {
        println!("Enter you guess: ");
        guesses += 1;

        // user input 
        let mut guesingNumberString: String = String::new();
        std::io::stdin().read_line(&mut guesingNumberString).expect("Failed to read line");

        // parsing from string to i32 and handling error
        let guess: i32 = match guesingNumberString.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        let difference: i32 = (number - guess).abs();

        // based on the difference, output a msg for the user
        match difference {
            0 => println!("Correct!"),
            1 ..= 10 => println!("Almost!"),
            11 ..= 25 => println!("Getting there."),
            26 ..= 50 => println!("Nowhere near."),
            _ => println!("Nope.")
        }

        // using cmp::Ordering to ouput msg to the user, if its correct the correct = true;
        match guess.cmp(&number) {
            std::cmp::Ordering::Less => println!("{} is too low! Try again!", guess),
            std::cmp::Ordering::Greater => println!("{} is too high! Try again!", guess),
            std::cmp::Ordering::Equal => {
                println!("You got it!!");
                println!("You try {} times", guesses);
                correct = true;
            }
        }

    }
     
}