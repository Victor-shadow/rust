fn main() {
    let a = 1;
    let  b = 2;

    //Bitwise AND, 0 (01 && 10 -> 00)
    let c = a & b;

    //Bitwise OR, 3 (01 || 10 -> 11)
    let d =  a | b;

    //Bitwise exclusive OR, 3 (01 != 10 -> 11)
    let e = a ^ b;
    //Left shift, 4
    let f = a << b;

    // 16 a -> '01' + "0000" -> '10000'
    let f2 = a << 4;

    //Right Shift, 0
    let g = a >> b;

    //0 '11111' -> remove 4 bits from the end -> '1'
    let g2 = 31 >> 4;

    //bitwise not
    let h = !a;

    println!("
     a : {a}, b : {b},
     c: {a} & {b} is {c},
     d: {a} | {b} is {d},
     e: {a} ^ {b} is {e},
     f: {a} << {b} is {f},
     f2: {a} << 4 is {f2},
     g: {a} >> {b} is {g}
     g2: {a} >> {b} is {g2}
     h: !{a} = {h}",
    a=a, b=b, c=c, d=d, e=e, f=f, f2=f2, g=g, g2=g2, h=h)

 }