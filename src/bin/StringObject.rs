fn main(){
    //Creates an empty string object
    let empty_string = String::new();
    println!("length is {}", empty_string.len());

   //Creates a string with some default value passed as a parameter to the from() method
    let string_object = String::from("Rust is a Statically typed programming language");
    println!("length is {}", string_object.len());
}