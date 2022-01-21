pub fn start(){
    let person: (&str, &str, i8) = ("Ali", "programmer", 32);
    println!("{:?}", person);
    println!("{} is a cool {} year old {} ",person.0, person.2, person.1)
}