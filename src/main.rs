// Standard Library for input and output
use std::io;
use rand::Rng;

fn main(){
    println!("Welcome to Guessing the number!!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number generated is : {secret_number}");

    println!("Kindly enter the number you guess!!");

    // Creating a new mutable variable
    let mut guess: String = String::new(); // new is associate function which creates a new empty string
    
    // input functions calling
    io::stdin()
            .read_line(&mut guess)
            .expect("The number should be an integer");
    
    println!("The number you guessed was {guess}");
}