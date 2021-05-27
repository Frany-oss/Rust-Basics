// List all possible combinations of letters in a 4-letter word.
// Eg 'TEST' can be unscrambled as TEST,TETS,TSET,TSTE,TTSE,TTES

use std::io;


pub fn permutations(word: String) -> Vec<String> {
    let length = word.len();
  
    if length <= 1 {
      // need return keyword for early return
      // otherwise error: expected () found String
      return vec![word];
    }
  
    // remove first character
    let trimmed = word.chars().skip(1).collect();
  
    // find all permutations of remaining letters
    let perms = permutations(trimmed);
    let current_char = word.chars().nth(0).unwrap();
    let mut result = Vec::new();
  
    // reinsert first letter in every possible place
    for perm in &perms {
      for i in 0..&perms.len() + 1 {

        let front: String = perm.chars().take(i).collect();
        let rest: String = perm.chars().skip(i).collect();
        result.push(format!("{}{}{}", front, current_char, rest));

      }
    }
  
    result
  }

fn main() {
    // get input
    println!("Enter a word (max 6 letters): ");

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();

    input_text = input_text.trim().to_string();

    println!("{:?}", permutations(input_text));

}
