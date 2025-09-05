use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main(){
   assert_eq!(
       Point { x: 1, y: 0} + Point {x: 2, y: 3},
       Point {x: 3, y: 3}

   );
}
//The add method adds the x values of two Point instances and the y values of two Point instances
//to create a new Point. The Add trait has an associated type named Output that determines the type returned from the add method