fn main(){
    let tuple:(i32, f64, &str) = (2010, 2.0,"Rust is a statically typed programming language");
    println!("The integer value in the tuple is {:?}", tuple.0);
    println!("The float point value in the tuple is {:?}", tuple.1);
    println!("The Strings value in the tuple is {:?}", tuple.2);


}