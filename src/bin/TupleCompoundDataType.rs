fn main() {
    //create a tuple using the tup keyword
    let tup = (500, 10.0, true);

    let (x, y, z) = tup; //destructuring, breaking a tuple of different elements into individual
    //components

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
}
//The program first creates a tuple and binds it to the variable tup
//It then uses a pattern with let to take tup and turn it into three separate variables, x, y, z
//The program prints the value of the individual elements in the tuple