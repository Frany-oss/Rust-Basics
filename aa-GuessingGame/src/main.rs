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

    let mut correct: bool = false;
    let mut guesses: i8 = 0;

    while !correct {
        println!("Enter you guess: ");
        guesses += 1;

        let mut guesingNumberString: String = String::new();
        std::io::stdin().read_line(&mut guesingNumberString).expect("Failed to read line");

        let guess: i32 = match guesingNumberString.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        let difference: i32 = (number - guess).abs();

        match difference {
            0 => println!("Correct!"),
            1 ..= 10 => println!("Almost!"),
            11 ..= 25 => println!("Getting there."),
            26 ..= 50 => println!("Nowhere near."),
            _ => println!("Nope.")
        }

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