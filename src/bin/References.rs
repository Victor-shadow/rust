fn main(){
    let s = String::from("Rust");
    let len = calculate_length(&s); //the &s creates a reference  that refers to the value of s but does not own it
    //the value it points to will not be dropped when the reference stops being used

    println!("The length of '{s}' is {len}")
}

fn calculate_length(s : &String) -> usize{ //s is a reference to String
    //the signature of the function uses & to indicate the type of the parameter
    //usize represents a size of memory
    s.len() //an expression

} //here s goes out of scope, but because s does not have ownership of what it refers to , the String is not dropped
//& - represent some references to  a particular value without taking ownership