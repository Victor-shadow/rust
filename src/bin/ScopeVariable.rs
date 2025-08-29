fn main() {
    //scope of a variable is determined by the opening '{' and where there is '}' for the variable
    let x = 1000;
    let z = 3000;//outer scope(main scope of the function) for the variable x
    {
        let y = 2000; //inner scope for variable y
        println!("The value of x is {} and the value of y is {}", x, y);
    }

    println!("The value of x is {} and the value of z is {}", x, z);

}