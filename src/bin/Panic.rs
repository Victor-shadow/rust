fn main(){
    let no = 1;

    //try with odd and even
    if no % 2 == 0 {
        println!("The number is even");
    } else {
        panic!("Not even");
    }
    println!("End of main");
}