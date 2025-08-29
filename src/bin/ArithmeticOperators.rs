fn main() {
    let x = 20;
    let y = 4;
    let mut result:i32;

    //addition of two operands
    result = x + y;
    println!("Sum: {} ", result);

    //difference of the values
    result = x - y;
    println!("Difference: {} ", result);

    //product of the values
    result = x * y;
    println!("Product: {} ", result);

    //quotient of the values(Divide left operand by the right one and return the quotient)
    result = x / y;
    println!("Quotient: {} ", result);

     //remainder(Divide the left operand by the right one and return the remainder)
    result =  x % y;
    println!("Remainder: {} ", result)
}