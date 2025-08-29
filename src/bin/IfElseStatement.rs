fn main(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("numbr is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//When this program executes, it checks each if expression in turn and executes the first body
//for which the condition evaluates to true
//Rust only executes the block for the first true condition