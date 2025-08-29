fn main(){
    match is_even(5) {
        Some(data) => {
            if data == true {
                print!("Even no")
            }
        },

        None => {
            println!("not even");
        }
    }
}

fn is_even(num: i32) -> Option<bool>{
    if num % 2 == 0{
        Some(true)
    } else {
        None
    }
}