fn main(){
    let mut s = String::from("hello");
    s = String::from("world"); //nothing refers to the original value on the heap

    println!("{s}, world!");
}
//when a new value is completely assigned to an existing variable, Rust calls drop and free's the original value memory immediately