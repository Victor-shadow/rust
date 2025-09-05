fn main(){
    let list = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!("Before definition of a closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}") ;
    println!("Before calling closure: {list:?}");
    only_borrows();

    println!("After calling closure: {list:?}");
}
//A variable can bind to a closure definition, and later the closure can be called by using the variable
//name and parenthesis as if the variable name were a function name
//Because one can have immutable references to list at the same time, list is still accessible from the code before
//the closure definition, after the closure definition but before the closure is called, amd after the closure is called