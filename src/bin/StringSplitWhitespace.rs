fn main(){
    //the split_whitespace() method  splits the input string into different strings
    //it returns an iterator (values)
    let output = "Rust is a safe and low level programming language".to_string();
    let mut i = 1;

    for token in output.split_whitespace(){
        println!("token {} {}", i, token);
        i += 1;
    }
}