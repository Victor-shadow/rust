struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { //&self is a reference to the Self instance //it borrows the Rectangle values immutably
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main(){
  let rect1 = Rectangle{
      width: 100,
      height: 700,
  };

  let rect2 = Rectangle{
      width: 200,
      height: 800,
  };

  let rect3 = Rectangle{
      width: 100,
      height: 200,
  };

  println!("The area of rect1 is {}", rect1.area());
  println!("the area of rect2 is? {}", rect1.can_hold(&rect2));
  println!("The area of rect3 is? {}", rect1.can_hold(&rect3));
}
