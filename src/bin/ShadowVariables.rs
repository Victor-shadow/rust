fn main(){
    let x = 1000;

    let x = x + 1;
    {
        let  x = x * 100;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope: {x}");
}
//This program first binds x to a value of 1000
//Then it creates a new variable x by repeating let x  = taking original value and adding 1 so that
//the value of x is now 1001
//Then within an innerscope created with curly braces, the third let statement shadows the second variable x
//and creates a new variable by multiplying the previous value by 100 to give x a new value
//When th scope is over, the inner shadowing ends and x returns to be 1001