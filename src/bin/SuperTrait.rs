use std::fmt;

//An OutlinePrint Trait with an outline_print method that prints a given value formatted so that it is in framed asterisks
//That is given a Point Struct that implements the standard library trait Display to result in (x, y)
//when we call outline_print method on a Point instance hat has 1 for x and 3 for y
//In the implementation of outline_print method, we want to use the Display's trait functionality
//Therefore one needs to specify that the OutlinePrint trait will work only for types that also implement Display
//and provide functionality that OutlinePrint Needs
//Because we have specified that OutlinePrint requires the Display trait , we can use the to_string method
//that is automatically implemented for any type that implements Display
//If one tries to use to_string without adding a colon and specifying the Display Trait after the trait name
//we get an error saying that the method named to_string was found for the type &self in the current scope
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("* {} *", " ".repeat(len + 2));
        println!("* {} *", " ".repeat(len + 2));
    }
}

//struct that implements Display and OutlinePrint
struct Point {
    x: i32,
    y: i32,
}
//implement Display so point can be printed as x and y
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
    }
}

//Implement OutlinePrint for Point
impl OutlinePrint for Point {

}
fn main(){
   let p = Point { x: 1, y: 3};
    p.outline_print();
}