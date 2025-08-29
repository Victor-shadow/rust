fn main(){
    //implementation of th copy trait
    let x = 5; //the value 5 is bound to variable x
    let y = x; //the value 5 stored in x is copied to y. Both x and y are valid
    //the two values are pushed to the stack since integers are simple values with a known and fixed size
    println!("x = {}, y = {}", x, y); //works fine
}