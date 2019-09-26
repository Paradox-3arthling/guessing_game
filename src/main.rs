use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to the guessing game :D");
    println!("Enter your guess, press <q> to stop guessing.");
    let secret_number = rand::thread_rn
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");    
    println!("You guessed: {}", guess);
}
