use std::io;

fn main() {
    new();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read number");
    println!("You guessed: {guess}");  
}
fn new() {
    println!("Guess A number")
}