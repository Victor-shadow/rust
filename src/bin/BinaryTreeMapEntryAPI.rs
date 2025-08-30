use std::collections::BTreeMap;

fn main(){
    let words = vec!["Java", "kotlin", "scala", "ruby", "Rust", "10", "10", "20", "30"];
    let mut count = BTreeMap::new();
    for  word in  words {
       *count.entry(word).or_insert(0) += 1;
    }

    for (word, frequency) in &count {
        println!("{}:  {}", word, frequency);
    }
}