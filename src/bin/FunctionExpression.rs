fn main(){
    let y =
        { //start of the expression is a block that evaluates to 4 and that value gets bound to y as part of the
            //let statement

        let x = 3;
        x + 1 //expressions do not include a semicolon
            //a semi-colon added at the end of an expression turns it into  a statement and then a value is not returned
    };

    println!("The value of y is: {}", y);
}