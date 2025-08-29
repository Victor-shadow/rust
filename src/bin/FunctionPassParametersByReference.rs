//mutable referencing and de-referencing
fn main(){
    let mut number: i32 = 100; //declare a mutable variable number with the mut keyword
    mutate_number_to_zero(&mut number); //a mutable reference to the variable is passed to the second invoked fn
    println!("The value of number is:{}", number)
}
//this function is given permission to modify number directly. mutate_number_to_zero(&mut number)
fn mutate_number_to_zero(param_no:&mut i32){
    *param_no = 0; //de reference
}

//*operator(accesses the actual value behind the reference)  is used to access a value (of a particular parameter)stored in the memory location that variable param_no points to
