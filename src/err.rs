use std::fs::File;
use std::io::ErrorKind;

pub fn work(){
    let cool_file: Result<File,_> = File::open("hello.txt");

    // This whole match statement works like the unwrap keyword 
    let f = match cool_file{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=> match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(err)=>panic!("Could not create a file, Error:{}", err)
            },
            others=>{panic!("Other Error:{:?}", others)}
        }
        
    };
    println!("{:?}", f);
    /*
    let another_file:Result<File,_>= File::open("hello.txt");
    let nother_file = match another_file{
        Ok(file)=>file,
        Err(error)=>panic!("Cant open file {}", error) 
    };
    */
    
    
}