mod midterms;

fn main() {
    println!("Hello, world!");

    let mut input_midterms = String::new();
    let mut input_homework = String::new();
    let mut input_finals   = String::new();

    println!("Enter your midterm's grade: ");
    std::io::stdin().read_line(&mut input_midterms).unwrap();
    let midterm = input_midterms.trim().parse::<f32>().unwrap();

    println!("Enter your homework's grade: ");
    std::io::stdin().read_line(&mut input_homework).unwrap();
    let homework = input_homework.trim().parse::<f32>().unwrap();

    println!("Enter your final's grade: ");
    std::io::stdin().read_line(&mut input_finals).unwrap();
    let finals = input_finals.trim().parse::<f32>().unwrap();

    let mut exams = midterms::Exams::new( midterm, homework, finals);

    exams.calculate();

}
