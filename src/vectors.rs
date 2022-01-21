// Vectors are growable arrays
// They are stack allocated
// This is same as saying from discord.commands import package
use std::mem::{size_of_val};

pub fn start(){
    let mut greatest: i32 = 0;
    let mut numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let numbers_array:[i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let size_of_vector = size_of_val(&numbers);
    let size_of_array = size_of_val(&numbers_array);
    let difference = if size_of_vector > size_of_array {size_of_vector-size_of_array }else{size_of_array-size_of_vector};
    println!("{:?}", numbers);
    println!("First Element:{:?}", numbers[0]);
    println!("Last Element:{:?}", numbers[9]);
    println!("This vector occupies {} bytes", size_of_vector);
    println!("This array occupues {} bytes", size_of_array);
    println!("The difference between size is {}", difference);

    // Add to vector
    numbers.push(5);
    numbers.push(10);
    for i in numbers{
        if i > greatest{greatest = i}
    }
    println!("{}", greatest)
}