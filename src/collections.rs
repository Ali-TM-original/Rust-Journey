use std::collections::HashMap;

#[allow(dead_code)]
pub fn work(){

    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }


    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Float(0.3),
        SpreadSheetCell::Int(10),
        SpreadSheetCell::Text(String::from("Hello World"))
    ];

    match &row[1]{
        SpreadSheetCell::Int(i)=>println!("{}", i),
        _=>println!("Not an Integer")
    }

    
    let mut vec: Vec<i32> = vec![1,2,3,4,5];

    for i in &mut vec{
        *i += 50
        
    };
    println!("{:?}", vec)
    
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn done(){
    let mut s:String = String::from("Hello");
    s.push_str(" ♥");
    println!("{}",s);

    let s1: String = String::from("Hello");
    let s2: String = String::from("From Rust");
    let s3: String  = s1 + &s2; // Ownership of s1 passed to s3
    println!("S1 Is owner of  {} ", s2);

    // Get Bytes of UTF-8 growable strings
    for b in s3.bytes(){
        println!("{}", b)
    }

}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn hash_test(){
    let blue:String = String::from("Blue Team");
    let yellow:String = String::from("Yellow Team");

    let mut scores: HashMap<String, i32> = HashMap::new(); 
    scores.insert(blue , 10); // Now the hashmap owner blue and yellow
    scores.insert(yellow, 20);
    let teamname:String = String::from("TW69");
    let score: Option<&i32> = scores.get(&teamname);
    /*
    println!("{}", score.unwrap_or(&-0));
    for (key, value) in &scores{
        println!("key: {}, value:{}", key, value);
    }
    */

    let mut map: HashMap<String, i32> = HashMap::new();
    let cool_string:String = String::from("Hello this is my string");
    let mut counter:i32 = 1;
    for word in cool_string.split_whitespace(){
        let entry = map.entry(word.to_owned()).or_insert(0);       
        *entry += counter;
        counter+= 1;
    }
    println!("{:?}", map);
}