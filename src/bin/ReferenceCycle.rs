use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
//definition of an enum list and a tail method
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), //the second element in the Cons variant is RefCell<Rc<List>>
    //modification of the List value a Cons variant is pointing to
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main(){
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());


    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //Uncomment the next line to see that we have a cycle
    // it will overflow the stack
    //println!("a next item  = {:?} ",a.tail());
}
//we create a Rc<List> instance holding a List value in the variable a with the initial list of 5, Nil
//We then create a Rc<List> instance holding another list value in the variable value b
//that contains the value '10' and points to the list in a
//  modify  a so it points to 'b' instead of Nil, creating a cycle . We do that by using the tail method to get a reference
//to the RefCell<Rc<List>> in 'a', which we put in the variable link
//then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from a Rc<List> that holds a Nil value to the Rc<List> in b

//The reference count of Rc<List> instances in both a and b is 2 after we change the list in a to point to b
//At the end of main, Rust drop the value of variable b which decreases the reference count
//of the  b Rc<List> instance from 2 to 1
//The memory that Rc<List> has on the heap  won't be dropped at this point, because its reference count is 1 at 0
//Then Rust drops a  which decreases  the reference count  of the 'a  Rc<List> instance from 2 to 1 as well
//The instance 's memory can't be dropped either because the other Rc<List> instance still refers to it
//The memory allocated to the list will remain uncollected forever


