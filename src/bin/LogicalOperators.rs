fn main() {
    let a = true;
    let b = false;

    let c = !a;
    let d = a && b;
    let e = a || b;

    println!("a: {}, b: {}, c: !{0} is {}, d: {0} && {1} is {}, e: {0} || {1} is {}", a, b, c, d, e);
}