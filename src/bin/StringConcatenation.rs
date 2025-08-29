fn main(){
    let string_language = "Golang".to_string();
    let string_developer = "Rob Pike".to_string();

    let total = string_developer + " developed " + &string_language; //string_language is a reference
    println!("{} ", total);
}