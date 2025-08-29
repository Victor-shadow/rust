fn string_characters(s: &String) -> &str{ //this function returns a slice &str - signifies string slice
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn main(){
    let output = String::from("Golang is a programming language");
    let display = string_characters(&output);
    println!("The output string is: {}", display);

}

//the index of the end of the word is retrieved by looking for the first occurrence of a space
//when a space is found, a string slice is returned using the start of the string and the index of the space
//as the starting and ending indices
//when the string_slices is invoked, we get back a single value that is tied to the underlying data
//the value is made up of a reference to the starting point of the slice and the number of elements in the 