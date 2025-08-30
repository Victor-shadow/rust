use std::collections::HashSet;
use std::hash::RandomState;

fn main (){
    //Default Constructor
    let set1: HashSet<i32> = HashSet::new();
    //with capacity
    let set2: HashSet<i32>  = HashSet::with_capacity(10);
    //from an array
    let set3 = HashSet::from([100, 200, 300, 400, 500]);
    //custom hasher
    let s = RandomState::new();
   // let set4  = HashSet::with_hasher(s);
}