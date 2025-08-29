fn main(){
    let mut a = 100;

    println!("a is {}", a);

    //arithmetic  addition and assignment, 100 + 100 = 200
    a += 200;
    print!("1: a+=200 is {}", a);
    //Arithmetic subtraction and assignment 100 - 10 = 90
    a -= 10;
    println!("2: a-=10 is {}", a);
    //Arithmetic multiplication and assignment 100 * 10 = 1000
    a *= 10;
    println!("3: a*=10 is {}", a);
    //Arithmetic division and assignment 100 / 10 = 10
    a /= 10;
    println!("4: a/=10 is {}", a);
    //Arithmetic remainder and assignment 100 % 2 =
    a %= 10;
    println!("5: a%=10 is {}", a);
    //Bitwise AND, and assignment, 10 && 10 -> 10  -> 2
    a &= 2;
    println!("6: a&=2 is {}", a);
    //Bitwise OR and assignment, 010 || 101 -> 111 -> 7
    a |= 5;
    println!("7: a|=5 is {}", a);
    //Bitwise exclusive or and assignment 111 ^= 010 -> 101 -> 5
    a ^= 2;
    println!("8: a^=2 is {}", a);
    //Left shift and assignment '101' + '0' -> 1010 - 10
    a <<= 1;
    println!("9: a<<=1 is {}", a);
    //Right shift and assignment, 1010- ->  10 ->
    a >>= 2;
    println!("10: a>>=2 is {}", a);


}