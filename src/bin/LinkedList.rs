use std::collections::LinkedList;

fn main(){
    //Create a new LinkedList
    let mut list = LinkedList::from([1, 2, 3]); //Initialize from an array

    //add elements to both ends
    list.push_front(0);
    list.push_back(4);

    //pop from the front
    assert_eq!(list.pop_front(), Some(0));

    //Iterate immutable
    for element in &list {
        print!("{}", element);
    }

    //check contents
    assert!(list.contains(&3));
}