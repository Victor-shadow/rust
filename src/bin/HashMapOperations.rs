use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);
    //scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(20);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //score has a value associated with the Blue team, and the result is 10
    //The get method returns an Option<&V>; if there is no value for that key in the hash map, get returns None
    //The program handles the option by calling copied to get an Option<i32> rather than Option<&i32>, then
    //unwrap_or to set score to 0 if scores does not have an entry for a key

    //iterate immutably
    for (key, values) in &scores {
     println!("{key}: {values}");
    }

}