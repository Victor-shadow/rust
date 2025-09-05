use std::fmt;
//A wrapper struct that holds an instance of Vec<T> that one can implement Display on Wrapper and use the Vec<T>

struct  Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //The implementation of display uses self.0 to access inner Vec<T> because wrapper is a Tuple Struct and Vec<T>  is the item at index 0 in the tuple
        //Then the functionality of the Display trait on Wrapper
        //The downside of using the technique is that the wrapper is a new type, so it does not have the methods of the value it is holding
        //We would have to implement all methods of Vec<T> directly on wrapper such that the methods delegate to self.0, which allows one to treat wrapper exactly like a Vec<T>
        //If we wanted the new type to have every method the inner type has , implementing the Deref trait on the wrapper to return the inner type
        //would be a solution
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main(){
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}