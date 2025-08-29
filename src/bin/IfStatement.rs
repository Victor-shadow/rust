fn main(){
    let number = 100;

    if number < 100 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }


    let num = 100;

    if num != 0{
        println!("The number is something else other than zero")
    }
}
//All if expressions start with the if keyword, followed by the condition,
//The condition  checks whether or not the variable(number) has a values less than 5
//the block of code is placed to execute if the condition is true
//Immediately after the condition inside th curly braces. Blocks of code associated with the conditions
//optionally, an else expression gives the program an alternative block of code to execute should the condition
//evaluate to false
//the condition  in this code  must be a bool. If the condition is not a bool, an error is generated