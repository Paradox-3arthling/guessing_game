use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Welcome to the guessing game :D");
    println!("Enter your guess, press <q> to stop guessing.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret no. is {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess:u32 = guess.trim().parse()
        .expect("please insert number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Greater => println!("Too high!"),
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("You guessed correct")
    }
}
