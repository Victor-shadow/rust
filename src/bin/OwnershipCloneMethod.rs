fn main(){
    let s1 = String::from("Rust");
    //heap data does get copied
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}