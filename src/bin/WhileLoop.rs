fn main(){
    let mut x = 0; //the counter variable
    while x < 10{
        x += 1;
        println!("inside loop x value is {}", x);
    }
    println!("outside loop x value is {}", x);

    let mut num = 10;
    while num!=0 {
        println!("{}", num);
        num = num -1;
    }

    let mut n = 1;

    while n <= 10 {
        println!("{}", n * 2);
        n = n + 1;

    }


}