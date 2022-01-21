// Arrays are of fixed lengths
// Vectors are growable Arrays
// Fixed arrays cant be added to but can be changed to
// Arrays are stack allocated

// This is same as saying from discord.commands import package
use std::mem::{size_of_val};

pub fn start(){
    let numbers:[i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", numbers);
    println!("First Element:{:?}", numbers[0]);
    println!("Last Element:{:?}", numbers[9]);
    println!("This array occupies {} bytes", size_of_val(&numbers))
}