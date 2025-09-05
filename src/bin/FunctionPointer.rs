//a function add_one that adds one to its parameter
fn add_one(x: i32) -> i32 {
    x + 1
}
//the function do_twice takes two parameters: a function pointer to any function that takes  an i32 parameter and returns an i32, and one i32 value
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
//the main function calls do_twice with the arguments add_one and 5
fn main(){
    //the do_twice function calls the function f twice passing it an arg value adds two function call results together
    let output = do_twice(add_one, 5);
    println!("The answer is: {output}");
}
//The code prints 'The answer is: 12'. We specify that the parameter f in do_twice is a fn
//that takes one parameter of type i32 and returns i32. We can then call f in the body of do_twice
//In main one can pass the function name add_one as an argument to do_twice function
