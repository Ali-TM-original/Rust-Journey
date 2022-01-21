// || = OR operator
// && = AND operator

pub fn start(){
    let age = 18;
    check(age);

}

fn check(age:i32){
    if  age >= 21{
        println!("You are eligible")
    }else if age == 18{
        println!("Ohh a newbie nice")
    }else{
        println!("Get a life kid")
    }
}