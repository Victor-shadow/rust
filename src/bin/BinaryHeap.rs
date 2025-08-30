use std::cmp::Reverse;
use std::collections::BinaryHeap;


fn main(){

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(7));
    min_heap.push(Reverse(2));

    assert_eq!(min_heap.pop(), Some(Reverse(2))); //smallest first


}