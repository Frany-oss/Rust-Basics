
pub struct Exams {
    midterms: f32,
    homework: f32,
    finals  : f32,
}

impl Exams {

    pub fn new( midterms: f32, homework: f32, finals: f32) ->Self {
        
        Exams {
        midterms: midterms,
        homework: homework,
        finals: finals,
        }

    }

    pub fn calculate(&mut self) {
        print!("The total grade of this semester is: {}", self.homework*0.4 + self.finals*0.4 + self.midterms*0.2);
    } 

}