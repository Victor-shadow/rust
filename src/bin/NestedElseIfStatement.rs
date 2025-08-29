fn main(){
    let number = 2;
    if number > 0{
        println!("{} is a positive number", number);
    } else if number < 0 {
        println!("{} is a negative number", number);
    } else {
        println!("{} is neither positive nor negative", number);
    }
}