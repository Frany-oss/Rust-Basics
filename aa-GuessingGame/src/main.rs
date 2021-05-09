#![allow(non_snake_case)]
use rand::Rng;

// simple function that return a random number between 0 and 100
fn get_random() -> i32 {
    let number = rand::thread_rng().gen_range(0..100);
    number
}

fn main() {
    println!("Hello, world!");

    // storing the random number in a varialbe for later use
    let number = get_random();

    let mut guesingNumberString = String::new();

    std::io::stdin().read_line(&mut guesingNumberString).expect("Failed to read line");
    let guesingNumber = guesingNumberString.parse::<i32>().unwrap();

    loop {
        print!("Enter your guess: ");

        if guesingNumber < number {
            println!("To low!");
        }
        else if guesingNumber > number{
            println!("To high!");
        }
        else {
            println!("Correct!!! the number was {}", number);
            break;
        }
    }
     
}