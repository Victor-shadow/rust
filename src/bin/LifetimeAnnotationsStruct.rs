struct ImportantExcerpt<'a>{
    part: &'a str, //string slice
}

fn main() {
    let novel = String::from("The Dolls house by Henrik Ibsen...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{ //an instance of the Important Excerpt
        part: first_sentence,
    };
    println!("{}", i.part);
}
//The struct has a single field part that holds the string slice(&str), which is a reference to a portion of a string
//As with generic data types, we declare the name of the generic lifetime parameter inside the angle brackets after the name of the struct so that it can be used
//in the body of the struct definition
//the lifetime annotation means that the instance of the Struct Name cannot outlive the reference it holds in its field
//the main function has created an instance of the Important Excerpt that holds a reference to the first sentence of the String owned by the variable novel
//the data in the novel exists before the ImportantExcerpt instance is called
//novel does not go out of scope until after the ImportantExcerpt goes out of scope, so the reference in ImportantExcerpt instance is valid

