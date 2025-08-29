fn main(){
    //individual characters in a String can be accessed using the char method
    let string = "Rust".to_string();

    for s in string.chars(){
        println!("{}", s)
    }
}