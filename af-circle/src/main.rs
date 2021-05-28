//use std::io;

/*
the area of a circle = pi*r^2
radio = r
diameter = 2r
*/

const PI:f32 = 3.14159;

fn circle(number: f32, input: &str) {

    // if the input is radio, then we need to calculate the area and the diameter of the circle
    if input.trim() == "radio" {
        println!("diameter: {} and area: {}", number*number, PI*number.powi(2));

    // if the input is diameter, then we need to calculate the area and the radio of the circle
    } else if input.trim() == "diameter" {
        println!("radio: {} and area: {}", number / 2.0, PI*number.powi(2));

    // if the input is area, then we need to calculate the radio and the diameter of the circle
    } else {
        let mut radio = number / (PI);
        radio = radio.sqrt();
        println!("radio: {} and diameter: {}", radio, radio * 2.0);
    }
}

fn main() {
    println!("Enter (enter the full word) and you will get the other two: ");
    println!("1) area");
    println!("2) radio");
    println!("3) diameter");

    // input of the three options
    let mut input_option = String::new();
    std::io::stdin().read_line(&mut input_option).unwrap();
    
    println!("Enter the value of the {}", input_option);

    // input for the number of the area/radio/diameter
    let mut input_number = String::new();
    std::io::stdin().read_line(&mut input_number).unwrap();

    // trim() to remove the blankspaces and backslashes. parse()::<>() parse a string into de type in <> 
    let number = input_number.trim().parse::<f32>().expect("Error...");

    // call the function
    circle(number, &input_option);

}
