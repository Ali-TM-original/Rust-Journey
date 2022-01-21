use rand::Rng; // Random Module
use std::io; // For input output
use std::cmp::Ordering; // For Comparison


pub fn guessinggame(){
    let number_to_guess: u32 = rand::thread_rng().gen_range(1..101);
    let mut user_guess = String::new();
    println!("Number to guess is:{} ",number_to_guess);
    loop{
        println!("Please Enter your guess");
        let stdio = io::stdin();
        stdio.read_line(&mut user_guess).expect("Failed to read line");

        let user_guess:u32 = match user_guess.trim().parse(){
            Ok(num)=>num,Err(_)=>continue,
        };
        println!("You guessed:{}", user_guess);

        match user_guess.cmp(&number_to_guess){
            Ordering::Less=>print!("Too Small"),
            Ordering::Greater=>println!("Too Large"),
            Ordering::Equal=>{
                println!("You Won");
                break;
            }
        }
    }
}