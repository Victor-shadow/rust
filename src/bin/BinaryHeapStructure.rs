use std::collections::BinaryHeap;

fn main(){
    let mut heap = BinaryHeap::new();
    heap.extend([1, 2, 3, 4, 5].iter().cloned());

    while let Some (max) = heap.pop(){
        println!("{}", max); //prints the elements from the heap

    }
}