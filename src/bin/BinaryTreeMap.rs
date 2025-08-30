use std::collections::BTreeMap;

fn main(){
    let mut map = BTreeMap::new();
    map.insert(100, "a");
    map.insert(200, "b");

    //access
    assert_eq!(map.get(&100), Some(&"a"));
    assert_eq!(map.get(&200), Some(&"b"));

    //iteration
    for(k, v) in &map{
        println!("{k}: {v}");
    }


}