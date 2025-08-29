fn main(){
    another_function(1000);
    print_labeled_measurement(100, 'A');
}

//the declaration of another function has a parameter named x. The type of x is specified as i32
//When 5 is passed as an argument to the function, the println! macro puts 5 where the pair of curly
//braces containing x  was in the format string

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label:char){
    println!("The measurement is: {value} {unit_label}")
}