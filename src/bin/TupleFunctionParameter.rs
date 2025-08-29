fn main(){
    let tup:(i32, bool, f64) = (1000, true, 2000.0);
    print_elements(tup);
}
//pass the tuple as a parameter
fn print_elements(x:(i32, bool, f64)){
    println!("Inside print method");
    println!("{:?},", x);
}