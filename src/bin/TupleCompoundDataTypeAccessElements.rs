fn main(){
    let x: (i32, f64, u8) = (500, 10.0, 2);

    let one = x.0;
    let two = x.1;
    let three = x.2;

    println!("The value of one is {}", one);
    println!("The value of two is {}", two);
    print!("The value of three is {}", three);
}
//the program creates a tuple x and then accesses elements of the tuple using their respective indices
//The first index in a tuple is 0