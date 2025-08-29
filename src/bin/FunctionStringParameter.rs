fn main(){
    let name: String = String::from("Rust Programming Language");
    display(name);
    //cannot access name after display
}

fn display(param_name:String){
    println!("param_name value is: {}", param_name);
}