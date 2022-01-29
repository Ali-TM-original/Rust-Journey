//Traits are an abstract definition of shared behavior amongst different types

pub struct NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String,
}

pub struct Tweet{
    pub username: String,
    pub reply: bool,
    pub content: String,
    pub retweets: bool,
}

pub trait Summary{
    fn summarize(&self)-> String;
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{}, {}", self.author, self.headline)
    }
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{}, {}", self.username, self.content)
    }
}

pub fn work(){
    let tweet:Tweet= Tweet{
        username:String::from("@Ali"),
        reply:true,
        content:String::from("This is my cool content"),
        retweets:false,
    };
    let news:NewsArticle = NewsArticle{
        author:String::from("@Ali"),
        headline:String::from("Rust Lang"),
        content:String::from("Rust is cool but has a steep learning curve")
    };

    println!("Summary of Tweet: {}", tweet.summarize());
    println!("Summary of News: {}", news.summarize());
}