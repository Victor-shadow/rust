fn main(){
    //while true
    let mut x = 0;
    loop{
        x = x + 10;
        println!("x={}", x);

        if x==100{
            break;
        }
    }
}
//the break statement is used to take control out of a construct
//using break statement in a loop causes the program to exit the loop