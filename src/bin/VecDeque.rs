use std::collections::VecDeque;

fn main() {
    // Create and use as a queue (FIFO)
    let mut queue = VecDeque::new();
    queue.push_back(100); // first in
    queue.push_back(200); // second in
    assert_eq!(queue.pop_front(), Some(100)); // first out

    // Create and use as a stack (LIFO)
    let mut stack = VecDeque::new();
    stack.push_front(1000); // pushed second
    stack.push_front(2000); // pushed first
    assert_eq!(stack.pop_front(), Some(2000)); // first out

    // Initialize from array
    let dq = VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Iterate
    for item in &dq {
        println!("{}", item);
    }
}