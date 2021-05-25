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

    let mut guesingNumberString: String = String::new();

    std::io::stdin().read_line(&mut guesingNumberString).expect("Failed to read line");
    let guesingNumber: i32 = guesingNumberString.trim().parse().unwrap();

    println!("{}", guesingNumber);
    println!("{}", number);

   /* loop {
        print!("Enter your guess: ");

        if guesingNumber < number {
            println!("To low!");
            process::exit(1);
            
        }
        else if guesingNumber > number{
            println!("To high!");
            continue;
        }
        else {
            println!("Correct!!! the number was {}", number);
            break;
        }
    }*/
     
}