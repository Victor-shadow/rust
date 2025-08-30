use std::collections::HashMap;

fn main() {
    let text = "Rust is a statically typed programming language";

    let mut map = HashMap::new();


    //the split_whitespace method returns an iterator over subslices, separated by whitespaces of the value in text
    // or_insert method returns a mutable reference (&mut V) to the value for the specified key
    //the mutable reference is stored in the count variable, so in order to assign that value, count is dereferenced, the mutable reference goes out of scope at the end of the for loop
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}