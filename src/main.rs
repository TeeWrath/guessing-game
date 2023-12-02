// Standard Library for input and output
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to Guessing the number!!");

    // Generating a random number in the rang of 1 to 100
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number generated is : {secret_number}");

    println!("Kindly enter the number you guess!!");

    // Creating a new mutable variable
    let mut guess: String = String::new(); // new is associate function which creates a new empty string
    
    // input functions calling
    io::stdin()
            .read_line(&mut guess)
            .expect("The number should be an integer");
    
    // Converting guess String to a 32 bit signed integer
    let guess : i32 = guess.trim().parse().expect("enter an integer number");

    println!("The number you guessed was {guess}");

    // Comparing generated number and guessed number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!")
    }
}