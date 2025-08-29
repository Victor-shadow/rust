fn main(){
    let mut s = String::from("hello world");
    let word = string_word(&s); //word gets the value 5
    s.clear(); //this empties the String, making it equal to ""
    //word still have the value of 5, but s no longer has any content, that can be used with the value 5
    //word is totally invalid
    println!("The first word is: {}", word);
}

fn string_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return  i;
        }
    }

    s.len()
}