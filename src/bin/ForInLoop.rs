fn main(){
    for x in 1..10 { //10 is not inclusive
        //the variable x is only accessible within the block
        if x == 5 {
            //The continue statement skips subsequent statements in the current iteration and takes the control back
            //to the beginning of the loop
            //It terminates the current iteration and starts the subsequent iteration
            continue;
        }
        println!("x is {}", x);

        for x in 1..20{
            println!("{}", x);
        }

    }
}