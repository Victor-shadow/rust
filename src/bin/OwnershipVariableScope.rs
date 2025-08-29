fn main(){ //variable is not valid here since it is not yet declared
    let variable = "Rust"; //variable is valid from this point going forward(the variable refers to a string literal where the value of the String is hardcoded into text)
    //manipulate s
    println!("variable: {}", variable);
}//the scope for the variable is now over and that variable is no longer valid

//When variable comes into scope it is valid
//it remains valid until it goes out of the scope