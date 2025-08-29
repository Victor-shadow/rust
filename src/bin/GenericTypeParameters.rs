struct Point<X1, Y1>{
x: X1,
y: Y1,
}

// Implement methods for Point<X1, Y1>
impl<X1, Y1> Point<X1, Y1> {
    // This method mixes self.x with other.y and returns a new Point
    // The new Point has type Point<X1, Y2>
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,     // x comes from self (type X1)
            y: other.y,    // y comes from other (type Y2)
        }
    }
}

//in the main fn a Point has been defined that has i32 for x with a value of 100 and f64 for y with a value of 200.5
//the p2 variable is a point struct that has a string slice for x with a value of "Rust" and a char of 'c' for y
//calling mixup on p for with the argument p2 gives p3 which has an i32 for x since x came from p1
//p3 variable has a char for y since y came from p2
fn main(){
    let p = Point{x: 100, y: 200.5};
    let p2 = Point{x:"Rust", y: 'c'};

    let p3 = p.mixup(p2);

    println!("p3.x = {}, p3.y =  {}", p3.x, p3.y);


}