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
        
}