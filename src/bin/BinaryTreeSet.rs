use std::collections::BTreeSet;

fn main(){
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([2, 3, 4]);

    let intersection = &a & &b; //{2, 3}
    let union = &a | &b; //{1, 2, 3, 4}
    let symmetric_difference = &a ^ &b; //{1, 4}
    let difference = &a - &b; //{1}

    println!("The value of a {:?} and b {:?} in intersection is {:?} , union is {:?}, symmetric difference is {:?}, and difference is {:?}", a, b, intersection, union, symmetric_difference, difference);

}