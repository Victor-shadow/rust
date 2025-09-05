fn main(){
    let mut list = vec![1, 2 ,3];
    println!("Before defining closure {list:?}");

    let mut borrow_mutably = || list.push(7);

    borrow_mutably();
    println!("After calling closure: {list:?}");
}
//Note that there is no longer a println! between the definition and the call of the borrow_mutably
//closure: when borrows_mutably is defined. it captures a mutable reference to list
//The closure is not used again after it is called, so the mutable borrow ends.
//Between the closure definition and the call an immutable borrow to print is not allowed because no other borrows are allowed
//when there is a mutable borrow