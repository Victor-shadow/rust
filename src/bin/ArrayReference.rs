fn main(){
    //declare a mutable array of seven string
    let mut arr = ["Java", "Kotlin", "Scala", "Rust", "Python", "Laravel", "Javascript"];
    update(&mut arr); //invoke the update function passing a mutable reference to the array
    println!("Inside main method {:?}", arr); //{:?} debug trait
}

fn update(arr: &mut [&str; 7]){ //[&str; 7] //an array of strings of 7 elements
    for i in 0..3{
        arr[i] = "Html";

    }
    println!("Inside update method {:?},", arr);
}