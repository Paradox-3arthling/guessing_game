use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Welcome to the guessing game :D");
    println!("To kill me, input <ctrl + c> .\\/_");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("the secret no. is {}", secret_number);
    loop{
        println!("Enter your guess, sQuire!! .\\/.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers FOOL! 00");
                continue;
            }
        };        

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You guessed correct");
                break;
            }
         }
        }
}
