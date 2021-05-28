
pub struct Calculator {
    num_1: usize,
    num_2: usize,
}

impl Calculator {

    pub fn new(num_1: usize, num_2: usize) -> Self {

        Calculator {
            num_1: num_1,

            num_2: num_2
        }
    }

    pub fn cal(&mut self, chosen: &str) {
        
        match chosen {
            "1" => println!("The multiplication of {} * {} is {}", self.num_1, self.num_2, self.num_1 * self.num_2),

            "2" => println!("The division of {} / {} is {}", self.num_1, self.num_2, self.num_1 / self.num_2),

            "3" => println!("The sum of {} + {} is {}", self.num_1, self.num_2, self.num_1 + self.num_2),

            "4" => println!("The subtraction of {} - {} is {}", self.num_1, self.num_2, self.num_1 - self.num_2),
            
            _ => println!("Exit..."),
        }
    }
}