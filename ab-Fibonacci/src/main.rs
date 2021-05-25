#![allow(non_snake_case)]

// fibonacci sequence with recursion (using match)

fn fib(number: i32) -> i32 {
    match number {
        0 => 1,
        1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}

fn main() {
    println!("Hello, world!");

    // user input 
    let mut fibNumberString: String = String::new();
    std::io::stdin().read_line(&mut fibNumberString).expect("Failed to read line");

    // parsing from string to i32 and handling error
    let fibNumber: i32 = match fibNumberString.trim().parse() {
        Ok(fibNumber) => fibNumber,
        Err(_) => 0,
    };

    println!("The fibonacci sequence of {} is: {} ",fibNumber,  fib(fibNumber));
}
