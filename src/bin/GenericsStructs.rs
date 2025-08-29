//Multiple generic parameters
#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Debug)]
//a point struct where x and y are both generic
struct Point<T, U> {
    x : T,
    y : U,

}

fn main(){
    let both_integer = Point {x : 100, y :200};
    let both_float = Point{ x: 1.1, y : 2.2};
    let integer_and_float = Point{x : 100, y : 200.0};


    println!("The value of integer and float is {:?}, {:?}, {:?}", both_integer, both_float, integer_and_float)
}