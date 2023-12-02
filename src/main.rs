use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing the number!!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number generated is : {secret_number}");

    loop {
        println!("Kindly enter the number you guess!!");
        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("The number should be an integer");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, // error handling
            Err(_) => continue
        };

        println!("The number you guessed was {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
