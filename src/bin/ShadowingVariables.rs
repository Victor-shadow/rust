#[allow(unused_variables)]
fn main(){ //outer scope for variable x
    //shadowing of variables: declaration of a new variable with the same name as the previous one
    let x = 100;
    let y: &str = "Rust is a statically typed language";

    { //inner scope for the  variable x - this variable shadows the first variable x declared in the
        //outer scope
        let x = 200;
        assert_eq!(x, 200);
    } //end of the inner scope for the variable x

    assert_eq!(x, 100); //the value of x  in this scope is 100

    let x = 300;
    let y = 400;
    println!("{}, {}", x, y);

} //end of outer scope