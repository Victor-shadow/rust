fn main(){
    let list_of_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //Use of function to_string defined in the ToString trait, which is the standard library that has implemented for any type that implements Display
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);
}