fn main(){
    let number: i32 = 100; //a variable is declared to an initial value of 100
    mutate_no_to_zero(number); //the variable is passed as a parameter by value to another invoked function
    println!("The value of the number is: {}", number);
}

fn mutate_no_to_zero(mut param_no:i32){ //the function which changes the value to zero
    param_no = param_no*0;
    println!("param_no value is :{}", param_no);
}

//note: after function call the control returns to the main method hence the value will be the same