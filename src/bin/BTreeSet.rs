use std::collections::BTreeSet;

fn main(){
    let mut set = BTreeSet::new();
    set.insert(300);
    set.insert(100);
    set.insert(500);
    set.insert(700);
    set.insert(900);

    //elements are automatically sorted
    for element in &set {
        println!("{element}"); //prints in ascending order
    }
    //check membership
    assert!(set.contains(&300));

    //remove element
    set.remove(&100);
}