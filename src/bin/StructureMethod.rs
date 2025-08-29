

//define the dimensions of a rectangle
struct Rectangle {
 width: u32, height: u32
}
//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self) ->u32 {
        //use the .operator to fetch the value of the field via the self keyword
        self.width * self.height
    }
}
fn main() {
 //instantiate the struct
    let small = Rectangle{
        width: 100,
        height: 300
    };
    //print the rectangle area
    println!("width is {} height is {} area of Rectangle is {}", small.width, small.height, small.area())
}