use std::io;

fn main(){
    println!("Welcome to Guessing the number!!");

    println!("Kindly enter the number you guess!!");

    let mut guess = String::new();
    
    io::stdin()
            .read_line(&mut guess)
            .expect("The number should be an integer");
    
    println!("The number you guessed was {guess}");
}