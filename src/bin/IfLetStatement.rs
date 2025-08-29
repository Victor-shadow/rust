fn main(){
    let condition = true;
    let number = if condition{5} else {6};
    println!("The value of the number is: {number}"); //the number variable will be bound to a value based
    //on the outcome of the if expression
    //blocks of code evaluate to the last expression in them, and numbers themselves are also expressions
    //the value of the whole if expression depends on which block of code executes
}