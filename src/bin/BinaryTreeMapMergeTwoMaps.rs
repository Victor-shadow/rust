use std::collections::BTreeMap;

fn main(){
    let mut map1 = BTreeMap::from([(1, "a"), (2, "b"), (3, "c")]);
    let mut map2 = BTreeMap::from([(1, "Kotlin"), (2, "Scala"), (3, "Rust")]);

    let output = map1.append(&mut map2);
    println!("{:#?}", output);

}