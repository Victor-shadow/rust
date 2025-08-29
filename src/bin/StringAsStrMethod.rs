fn main(){
    let string_value = String::from("Rust is a Systems programming language");
    print_literal(string_value.as_str());
}
fn print_literal(str_value:&str){
    println!("display String literal {}", str_value);
}