use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person{
    name: String,
    age: u8,
}


fn main(){
    let mut people = HashSet::new();
    people.insert(Person{name: "James Gosling".to_string(), age: 70});
    people.insert(Person{name: "Denis Ritchie".to_string(), age: 50});
    assert_eq!(people.len(), 2);

}