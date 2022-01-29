// No Null val in Rust
// Use the option enum for error handeling specifically null values
#[allow(dead_code)]
enum Apiversion{
    V2(String, u8),
    V3(String, u8),
}

#[derive(Debug)]
#[allow(dead_code)]
enum PkrNote{
    Ten,
    Hundred,
    Thousand,
    FiveThousand,
}

#[allow(dead_code)]
enum Coin{
    Pakistan(PkrNote),
    India,
    Iran,
    Afghanistan,
}

fn convert_to_rs(coin:Coin)->u8{
    match coin{
        Coin::Pakistan(note) => {
            println!("Note is {:?}", note);
            3
        },
        Coin::India => 20,
        Coin::Afghanistan => 30,
        Coin::Iran =>50,
    }
}

fn optional_plus_one(x: Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i)=> Some(i+1),
    }
}

pub fn work(){
    /*
    let token:u8 = 12;
    let _api = api_version::V2(String::from("abc123" ), token); 
    let some_number :i8=32;
    let second_nuber :Option<i8>=Some(1);
    let new_var:i8 = some_number + second_nuber.unwrap_or(30);
    println!("{}", new_var);
    */
    let num_five: Option<i32> = Some(43);
    let ans = optional_plus_one(num_five);
    println!("{:?}", ans.unwrap());
    convert_to_rs(Coin::Pakistan(PkrNote::Hundred));
}
