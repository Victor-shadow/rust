fn five() -> i32 { //the function return type after the arrow
    5 //function return value(expression)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn main(){
    let x = five(); //the return value of a function
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");

}
//there are no function calls, macros or even let statements in the five function - just the number 5 itself
//that is a valid function in Rust