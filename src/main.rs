mod pkg; // Declare the `pkg` module

use pkg::homework1::Homework1; // Import the `Homework1` struct from the `pkg::homework1` module

fn main() {
    let field: u32 = 71;
    
    let mut hw1 = Homework1::new(field);
    
    hw1.q1();
    hw1.clear_values();
    
    hw1.q2();
    hw1.clear_values();
    
    hw1.q3();
    hw1.clear_values();
    
    hw1.q4();
    hw1.clear_values();
    
    hw1.q5();
    hw1.clear_values();
}
