enum List{
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons,Nil};
use std::rc::Rc;


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
//At each point in the program, where the reference count changes, the reference_count gets printed
//calling the Rc::strong_count function
//This function is named as strong_count rather than count because the Rc<T> type also has a weak count
//The Rc<List> in a has an initial reference count of 1; then each time clone is called
//the count goes up by one, When c goes out of scope, the count goes down by one
//we do not have to invoke a function to decrease the reference count like to call Rc::clone to increase the
//reference count: the implementation of the Drop trait decreases the reference count automatically when a Rc<T>
//value goes out of scope
//Note: When a and b go out of scope at the end of the main fn the count is then zero and the Rc<List> is cleaned up completely
//using Rc<T> allows a single value to have multiple owners, and the count ensures that the values remain  valid as long as the owners still exist