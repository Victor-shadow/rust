use std::collections::HashSet;

fn main(){
    let mut books = HashSet::new();
    books.insert("A Dance with Dragons".to_string());
    books.insert("Rust Book".to_string());

    if !books.contains("The Winds of Winter"){
        println!("We have other books, not that one. {}", books.len());
    }
    
    books.remove("the Odyssey");

    for book in &books{
        println!("{book}");
    }
}