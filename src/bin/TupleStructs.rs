
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("The value of origin is: {:?}", origin); //the specifier :? inside the curly braces tells the println! macro to use an output format called Debug
    println!("The value of black is: {:?}", black);


}

//note that black and origin values are of different types because they are instances of different
//tuple structs
//Each struct defined is of its own type, even though the fields within the struct might have the same
//types
//i.e a function that takes a parameter of type color cannot take Point as an argument, even though both types
//are made of three i32 values
//Otherwise tuple struct instances are similar to tuples in that one can destructure them into individual pieces
//and one can use a . followed by the index to access an individual value
//tuple structs require one to name the type of struct when you destructure them
//i.e let Point(x, y, z) = origin; to destructure the values in the origin point into variables named x, y, z