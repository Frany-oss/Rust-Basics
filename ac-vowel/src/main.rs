use std::io;

fn main() {
    println!("Hello, world!");

    println!("Enter some text: ");

    let mut input_text = String::new();

    io::stdin().read_line(&mut input_text).unwrap();

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut vowel_count: i32 = 0;
    let mut consonant: i32 = 0;

    for letter in input_text.chars() {

        if letter.is_alphabetic() {
            if vowels.contains(&letter) {
                vowel_count += 1;
            }
            else {
                consonant += 1;
            }
        } 
    }
    println!("Vowel count: {}", vowel_count);
    println!("Consonant count: {}", consonant);
}
