
pub fn rot13(line: &str) -> String {

    let mut converted = String::new();

    // see if each character of the input is valid or not
    for letter in line.chars() {
        if letter.is_alphabetic() {
            // if is a valid character, then we encrypt it with get_char_code()
            converted.push(get_char_code(letter));
        } else {
            // else we dont encrypt and leave it as it is
            converted.push(letter);
        }
    }

    converted
}

 // this function is being use before the crating, and thats why it appears as "not used" so I added allow(dead_code)
#[allow(dead_code)]
fn get_char_code(letter: char) -> char {
    match letter {
        'A' => 'N',
        'B' => 'O',
        'C' => 'P',
        'D' => 'Q',
        'E' => 'R',
        'F' => 'S',
        'G' => 'T',
        'H' => 'U',
        'I' => 'V',
        'J' => 'W',
        'K' => 'X',
        'L' => 'Y',
        'M' => 'Z',
        'N' => 'A',
        'O' => 'B',
        'P' => 'C',
        'Q' => 'D',
        'R' => 'E',
        'S' => 'F',
        'T' => 'G',
        'U' => 'H',
        'V' => 'I',
        'W' => 'J',
        'X' => 'K',
        'Y' => 'L',
        'Z' => 'M',
        'a' => 'n',
        'b' => 'o',
        'c' => 'p',
        'd' => 'q',
        'e' => 'r',
        'f' => 's',
        'g' => 't',
        'h' => 'u',
        'i' => 'v',
        'j' => 'w',
        'k' => 'x',
        'l' => 'y',
        'm' => 'z',
        'n' => 'a',
        'o' => 'b',
        'p' => 'c',
        'q' => 'd',
        'r' => 'e',
        's' => 'f',
        't' => 'g',
        'u' => 'h',
        'v' => 'i',
        'w' => 'j',
        'x' => 'k',
        'y' => 'l',
        'z' => 'm',
        _ => ' '
      }
}

// running a test to see if it encrypts correctcly
#[test]
fn it_converts() {
  let input = "Hello World!".to_string();
  let converted = "Uryyb Jbeyq!".to_string();
  assert!(rot13(&input) == "Uryyb Jbeyq!");
  assert!(rot13(&converted) == "Hello World!");
}