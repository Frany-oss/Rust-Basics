mod calculator;

fn main() {

    // number 1
    let mut input_num_1 = String::new();
    let mut input_num_2 = String::new();

    // input of the choice
    let mut input = String::new();

    println!("Welcome to the rust calculator!!");
    println!("-------------------------------");
    println!("Select the mathematic operation you what to use (just the number)..");
    println!("1) Multiplication");
    println!("2) Division");
    println!("3) Sum");
    println!("4) subtraction");
    println!("5) Exit");

    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    println!("-------------------------------");
    println!("Input the two numbers you want to {}", input);

    std::io::stdin().read_line(&mut input_num_1).unwrap();
    std::io::stdin().read_line(&mut input_num_2).unwrap();

    let num_1 = input_num_1.trim().parse::<usize>().unwrap();
    let num_2 = input_num_2.trim().parse::<usize>().unwrap();

    let mut calculator = calculator::Calculator::new(num_1, num_2);

    calculator.cal(&input);

}
