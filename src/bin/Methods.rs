#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn perimeter(&self) -> u32{
        self.width * 2 + self.height * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 600,
    };

    println!("The perimeter of the rectangle is {} px.",
        rect1.perimeter()
    );
}
//to define the function within the context of Rectangle, we start an impl (implementation) block
//for the rectangle. Everything within the impl block will be associated with the Rectangle type
//Then the perimeter function is moved within the impl curly brackets and change the first; in this case only the
// parameter to be self in the signature and everywhere within the body
//in main, where the perimeter function is called and &rect 1 is passed as an argument one can instead use the
//method syntax to call the perimeter method on the Rectangle instance
//The method syntax goes after an instance, a dot is added followed by the method name, parenthesis and any arguments

//In the signature for perimeter &self is used instead of &Rectangle
//within an impl block, the type self is an Alias for the type that the impl block is for
