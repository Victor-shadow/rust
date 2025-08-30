use std::collections::BTreeSet;

fn main(){
    let a = BTreeSet::from([1, 2,3]);
    let b = BTreeSet::from([2, 3, 4]);

    let set = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    //get the elements in range 3..7
    for elem in set.range(3..7){
        print!("{elem}");
    }

    for elem in set.range(5..){
        println!("{elem}");
    }

    //Union
    let union: Vec<_> = a.union(&b).cloned().collect();
    assert_eq!(union, vec![1, 2, 3, 4]);

    //Intersection
    let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    assert_eq!(intersection, vec![1, 2, 3, 4]);

    //differences
    let difference: Vec<_> = a .difference(&b).cloned().collect();
    assert_eq!(difference, vec![1]);
}