fn main(){
    let mut x = 0 ;

    for num in 0..21{
        if num % 2 == 0 {
            continue;
        }
        x = x + 1;
    }
    println!("The count of odd values between 0 to 20 is: {}", x);

}
//the loop will display the number of even values between 0 and 20
//it exits the current iteration if the number is even