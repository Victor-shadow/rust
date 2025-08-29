use std::fmt::Display;

struct Pair<T>{ //always implements the new fn to return a new instance
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y} //Self is a type alias for the type of the impl block(Pair<T>)
    }
}

impl <T: Display + PartialOrd> Pair<T> { //Pair<T> only implements the cmp_display method if its inner type T implements the traits
    fn cmp_display(&self){
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            print!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {

}
//By using a trait bound with an impl block that uses generic type parameters, one can implement methods
//conditionally for types that implement the specified traits