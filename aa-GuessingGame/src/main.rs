#![allow(non_snake_case)]
use rand::Rng;

// simple function that return a random number between 0 and 100
fn get_random() -> i32 {
    let number = rand::thread_rng().gen_range(0..100);
    number
}

fn main() {
    println!("Hello, world!");

    // 
    let number = get_random();
        
}
