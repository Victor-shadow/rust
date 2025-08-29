fn main() {
 //the trim() function removes leading and trailing spaces in a String
    //The function does not remove inline spaces
    let fullname = " Tutorials Point \r\n";
    println!("Before trim ");
    println!("length is {}", fullname.len());
    println!();
    println!("After trim");
    println!("length is {}", fullname.trim().len())

}
