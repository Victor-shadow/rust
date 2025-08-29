fn main(){
    //the mut keyword can be used to declare a mutable array
    let mut arr = ["Java", "Kotlin", "Scala", "Rust", "Python", "Laravel", "Javascript"];
    arr[1] = "Flutter";
    println!("arr elements are {:?}, ", arr);
}