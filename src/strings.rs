// Two types of strings
// 1>. immutable
// 2>. Growable

pub fn start() {
    let mut hello  = String::from("Hello ");
    //Get length of string
    println!("{}", hello.len());
    hello.push('W');
    println!("{}", hello);
    hello.push_str("TAB");
    println!("{}", hello);

    // To print capacity of a string use .capacity method on a string
    println!("Capacity is :{}", hello.capacity());

    // Trying out a foor loop
    // You cannot loop through a string you have to use chars
    for word in hello.to_lowercase().chars(){
        if word == 'w'{
        println!("{}", word);
        }
    }

    let mut lengthed_string = String::with_capacity(8);
    lengthed_string.push('y');
    lengthed_string.push('e');
    lengthed_string.push('s');
    println!("{}", lengthed_string);
}