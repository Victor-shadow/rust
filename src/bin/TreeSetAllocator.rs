use std::alloc::System;
use std::collections::BTreeSet;

fn main() {
    //Create the BTreeSet using the system allocator
    let mut set = BTreeSet::new();

    //insert elements
    set.insert(1);
    set.insert(2);

    //iterate and print
    for item in &set {
        println!("{}", item);

    }
}