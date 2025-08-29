fn main(){
    let n1 = "Android".to_string();
    let n2 = "Software Development Kit".to_string();
    //use of macro function format
    let n3 = format!("{} {}", n1, n2);
    println!("{}", n3);
}