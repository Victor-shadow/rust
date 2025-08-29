struct Rectangle {
    width:u32, //struct field1
    //struct field2
    height:u32,
}

fn main() {
    let rect1 = Rectangle{ //instance of Rectangle structure
        width: 30,
        height: 50,
    };
    println!(
        "The area of the Rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 { //an immutable reference(&Rectangle) is a borrow from the struct Rectangle for temporary modification
    //access the fields of the borrowed struct instance does not move the field values
    rectangle.width * rectangle.height
}