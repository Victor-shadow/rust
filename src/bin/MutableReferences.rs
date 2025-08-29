fn main(){
    let mut s = String::from("Rust");
    change(&mut s);
}

fn change(some_string: &mut String){
    some_string.push_str(" Programming Language");
    println!("{}", some_string);
}