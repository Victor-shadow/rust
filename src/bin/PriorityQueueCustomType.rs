use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Task {
    priority: u32,
    description: String,
}


fn main(){
    let mut queue = BinaryHeap::new();
    queue.push(Task { priority: 3, description: "Low Priority".into() });
    queue.push(Task { priority: 2, description: "Medium Priority".into()});

     println!("The largest priority is {}", queue.peek().unwrap().priority);

}