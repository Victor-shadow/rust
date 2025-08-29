

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main(){
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

}
//we create a value that is an instance of Rc<RefCell<i32> and store it in a variable named value
//so that one can access it directly later. Then a List is created in which  a Cons variant that holds a value
//One needs to clone value so both a and value have ownership of the inner value 5 rather than transferring ownership
//from value to a or having a borrow from a value
//we wrap the list 'a' in Rc<T> so that when lists b and c are created, they can both refer to 'a'
//After the lists a, b and c are created, 10 is added to value by calling borrow_mut on value, which uses an automatic dereferencing
//The borrow_mut method returns a RefMut<T> smart pointer, and one can use the dereference operator on it to change the inner value
