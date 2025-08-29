fn main(){
    let a = 500;
    let b = 600;
    //comparison operators
    //is equal to
    let c = a == b;
    //is not equal to
    let d = a != b;
    //is less than
    let e = a < b;
    //is greater than
    let f = a > b;
    //is less than or equal to
    let g = a <= b;
    //is greater than or equal to
    let h = a >= b;

    println!("a: {}, b: {},
    c: {0} == {1} is {},
    d: {0} != {1} is {},
    e: {0}< {1} is {},
    f: {0} > {1} is {},
    g: {0} <= {0} is {},
    h: {0} >= {0} is {}", a, b, c, d, e, f, g, h);
}