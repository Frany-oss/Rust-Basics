#![allow(non_snake_case)]

// fibonacci sequence with recursion
fn fib(number: i32) -> i32 {
    match number {
        0 => 1,
        1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}

fn main() {
    println!("Hello, world!");

    println!("{}", fib(10));
}
