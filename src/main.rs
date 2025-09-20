use rand::prelude::*;
use std::io;
fn main() {

    let number = thread_rng().gen_range(1..101);
    println!("Guess a number between 1 and 100");

     

    loop {
        let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);
    
    let ints = buffer.trim().parse::<i32>().expect("Invalid input: Please enter a valid integer");
        if ints < number{
            println!("Guess higher");
        }
        else if ints > number{
            println!("guess lower");
        }
        else{
            println!("You got it!");
            break;
        }
        
    }
    println!("The number was {}!", number);

}
 