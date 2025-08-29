#[allow (unused_variables)]
fn main(){
    let s1 = give_ownership(); //give_ownership moves its return value into s1

    let s2 = String::from("Rust");//s2 moves into scope
    println!("{s2}");

    let s3 = takes_and_gives_back(s2); //s2 is moved into
    //takes and gives back, which also moves its return value to s3
} //here s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped

fn give_ownership() -> String{ //give_ownership will move its return value to the function that invokes it

    let some_string = String::from("Scala"); //some_string comes into scope
    some_string //some string is returned and moves out to the calling function

}
//this function takes a string and returns a String
fn takes_and_gives_back(a_string: String) -> String{
    //a_string comes into scope
    a_string //a_string is returned and moves out of the calling function
}