enum List{
    Cons(i32, Rc<List>), //Rc<T> // a smart pointer for shared ownership
    Nil, //Rc<List> - allows multiple owners of the list
}

use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); //Rc::clone - increments the Reference count
    let c = Cons(4, Rc::clone(&a));

    println!("Reference count after b and c: {}", Rc::strong_count(&a));
    //Rc::strong_count(&a) //how many Rc pointers are referencing the same data

}
//we need to add the use statement to bring Rc<T> into scope because it is not in the prelude
//In the main method we create a list holding 5 and 10 and store it in a new Rc<List> in a
//Then when b and c is created, call the Rc::clone function and pass the reference to the Rc<List> in a as an argument
//The implementation of the Rc::clone instead of the clone method by Rust is not to make a deep copy of all the data like most
//implementations of clone do. the call Rc::clone only increments the reference count