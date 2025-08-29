#[derive(Debug)]
struct Rectangle {
    width: u32
}

impl Rectangle {
    fn width(&self) ->bool {
        self.width > 0
    }
}
fn main(){
    let rect1 = Rectangle{
        width: 1000,

    };

    if rect1.width(){
        println!("The rectangle width; it is {}", rect1.width);
    }
}