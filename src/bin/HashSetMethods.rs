use std::collections::HashSet;

fn main(){
    let a = HashSet::from([1, 2, 3]);
    let  b = HashSet::from([4,2,3, 4]);

    //Difference
    let diff: HashSet<_> = a.difference(&b).collect();
    assert_eq!(diff, [1].iter().collect());

    //union
    let union: HashSet<_> = a.union(&b).collect();
    assert_eq!(union, [1, 2, 3, 4].iter().collect());

    //Intersection
    let intersection: HashSet<_> = a.intersection(&b).collect();
    assert_eq!(intersection, [2, 3].iter().collect());

}