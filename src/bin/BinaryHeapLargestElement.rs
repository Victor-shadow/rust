use std::collections::BinaryHeap;

fn k_largest<T: Ord>(elements: Vec<T>, k: usize) -> Vec<T> {
    let mut heap = BinaryHeap::from(elements);
    let mut result = Vec::with_capacity(k);

    for _ in 0..k {
        if let Some(item) = heap.pop(){
            result.push(item);
        }
    }
    result
}


fn main() {
    let data = vec![10, 4, 7, 1, 9, 3, 8];
    let k = 3;
    let top_k = k_largest(data, k);
    println!("Top {} elements: {:?}", k, top_k);

}