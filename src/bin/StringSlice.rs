fn main(){
    let a1 = "Golang".to_string();
    println!("The length of the String is {}", a1.len());
    let a2 = &a1[1..5]; //borrowing(immutable reference)
    //fetches characters at 1, 2
    println!("{}", a2);
}