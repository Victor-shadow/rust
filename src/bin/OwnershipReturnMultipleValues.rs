fn main(){
    let s1 = String::from("Golang");
    //multiple values in Rust can be returned using a tuple
    let (s2, len) = calculate_length(s1);

    println!("the length of '{s2}' is {len}");
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); //len() returns the length of thr String
    (s, length)
}