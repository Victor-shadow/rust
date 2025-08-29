//the first_word function has a parameter of type String
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //convert the String to an array of bytes to check whether a value is a space

    for (i, &item) in bytes.iter().enumerate()  { //create an iterator over the array of bytes using iter() method
        //iter is a method that returns each element in a collection
        //enumerate method wraps the results of iter and returns each part of the element as part of a tuple instead
        //the first element of the tuple returned from enumerate is the index, the second element is a reference to the element
        //the for loop defines a pattern that has i for the index in the tuple  and &item for the single byte in the tuple
        //to get a reference to the element from .iter().enumerate() use & in the pattern
        //inside the for loop, search for the byte that represents the space by using the byte literal syntax
        //if a space is found, the position is returned, otherwise the length of the string is returned
        if item == b' '{
            return i; //return the index of the end of the word indicated by space
        }
    }

    s.len()
}
fn main(){
    let sentence = String::from("Rust is powerful");
    let index = first_word(&sentence);//sentence is passed as an immutable reference
    println!("The index of the first word boundary is: {}", index);

}