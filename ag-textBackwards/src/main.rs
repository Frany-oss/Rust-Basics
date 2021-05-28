
// lifetimes! we want output to live long enough to be printed
// since we're passing it in as a reference
// so the return String needs to live that long as well
// which means we also need to annotate reverse to show where the lifetime comes from

fn backwards_word<'a>(output: &'a mut String, input: &str, n: i32) -> &'a mut String {

    if n == 0 {
        return output;
    }

    output.push(input.chars().nth((n - 1) as usize).unwrap());
    backwards_word(output, input, n - 1)
}

fn main() {
    println!("Hello, world!");

    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();

    input_text = input_text.trim().to_string();

    let mut output = String::new();

    println!("{}", backwards_word(&mut output, &input_text, input_text.len() as i32));
}
