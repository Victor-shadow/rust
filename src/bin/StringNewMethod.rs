fn main(){
    let mut string  = String::new(); //provides a growable heap allocated String
    string.push_str("Rust is a Statically typed programming language");
    println!("{}", string);
}