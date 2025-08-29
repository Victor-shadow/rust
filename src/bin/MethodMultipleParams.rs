#[warn(dead_code)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    //this method takes an immutable borrow of another Rectangle as a Parameter
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 100,
        height: 10,
    };
    let rect3 = Rectangle{
        width: 50,
        height: 1000
    };

    println!("Can rec1 hold rect2? {}", rect1.can_hold(rect2)); //rect1.can_hold(&rect2) //an immutable borrow to rect2, an instance of Rectangle
    println!("Can rec1 hold rect3? {}", rect1.can_hold(rect3));
}