
/* Racecar */

fn palindrome(x: &str) -> bool {
    if x.to_string() == x.to_string().chars().rev().collect::<String>() {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let x = input.trim();

    println!("{}", palindrome(x));
    
}
