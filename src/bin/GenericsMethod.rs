struct Point<T, U>{
    x: T,
    y: U,
}

impl <T, U> Point<T, U>{
    //a method x is defined on the struct name(Point) that returns a reference to the data in the field x
    //T has to be declared just after impl so T can be used to specify that one is implementing methods on the type Point<T, U>
    //By declaring T and U as generic type after impl, Rust can identify the data type in the angle brackets
    //in the struct Point is now a generic type
    //if a method is written within an impl that declares a generic type, that method will be defined on any instance of the type
    //constraints can also be defined on generics when defining methods on the types
    fn x(&self) -> &T {
        &self.x
    }
}


fn main(){
    let p =Point{x : 100, y: 200};
    println!("p.x = {}", p.x());

}
