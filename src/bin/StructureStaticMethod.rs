//declare a structure
struct Point {
    x: i32,
    y: i32,
}
impl Point{
    //static method that creates objects of the Point Structure
    fn get_instance(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }

    //display values of the structure field
    fn display(&self){
        println!("x = {}, y = {}", self.x, self.y);
    }

}

fn main(){
    //Invoke the static method
    let output = Point::get_instance(100, 200);
    output.display();

}