use std::thread;

fn main(){
    let list = vec![1, 2, 3, 4, 5, 6];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
//we spawn a new thread, giving the thread a closure to run as an argument
//The closure body prints out the list
//The closure only captured list using an immutable reference because that is the least amount of access to list needed to print it
//Even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure
//by putting the move keyword at the beginning of the closure definition
//The new thread might finish before the rest of the main thread finishes, or the main thread might finish
//first, if the main thread maintained ownership of list, but ended before the new thread did and dropped list,
//the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure
//given to the new thread so the reference is valid. Try removing the move keyword, or using list in the main thread after the closure is defined to receive which compiler errors