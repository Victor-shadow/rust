fn main(){
    let string_type = "Kotlin is interoperable with Java".to_string();
    //the replace function takes two parameters - the first parameter is a string pattern to search for and the second parameter
    //is the new value to be replaced
    let string_type1 = string_type.replace("Kotlin", "Android");
    println!("{}", string_type1);

}