use std::io;

fn main() {
    println!("Guess a number !");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read number");
    prinln!("You guessed: {guess}");  
}
fn new() {
    println!("poop")
}