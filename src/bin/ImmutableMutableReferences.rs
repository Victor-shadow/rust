fn main(){
    let mut s = String::from("Rust language");
    let immutable_reference1 = &s; //no problem
    let immutable_reference2 = &s; //no problem
    println!("{immutable_reference1} and {immutable_reference2}");
    //Variables immutable_reference1 and immutable_reference2 will no longer by valid
    let mutable_reference = &mut s; //no problem
    println!("{mutable_reference}")
}
//the scopes of the immutable references end after the println!, where they are last used, which is before the mutable
//reference is created
//these scopes do not overlap