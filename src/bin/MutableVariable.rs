fn main(){
    //the mut keyword is used to make a variable mutable
    let mut x = 1000;
    println!("The value of x is: {}", x);
    x = 200;
    println!("The new value of x is: {}", x);

    let mut a = 100;
    a = a + 900;
    assert_eq!(a, 1000);
    println!("The new value of a is: {}", a);
}