#[allow(dead_code)]
pub struct User {
    username:String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub struct Rectangle{
    height:u32,
    width:u32,
}

impl Rectangle{
    fn area(&self, )-> u32 {
        self.height*self.width
    }

    fn perimeter(&self)->u32{
        2*(self.height+self.width)
    }
}

#[allow(unused_variables)]
pub fn work(){
    let mut user_one:User = User{
        username: String::from("Ali"),
        email: String::from("Ali@Ali.com"),
        sign_in_count: 100,
        active: true,
    };

    let name = user_one.username;
    user_one.username = String::from("Abdullah");
    let new_user = build_user(String::from("Mail@mail.com"), String::from("Umar"));
    let user_three:User = User{
        username: String::from("Test"),
        email: String::from("USERTHREE@EMAIL"),
        ..new_user
    };
    // If there is a sting in the struct old struct instant will be moved to new once copied
    println!("{}, {:?}", name, new_user.username);

    let rect:Rectangle= Rectangle{height:32, width:32 };
    println!("Area of rectangle is {} , perimeter of rectangle is {}", rect.area(), rect.perimeter());

}

fn build_user(email:String, username:String)->User{
    User{
        username: username,
        email:email,
        active:false,
        sign_in_count: 200, 
    }
}