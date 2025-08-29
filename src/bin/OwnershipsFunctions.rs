#[allow(unused_variables)]
fn main(){
    let s = String::from("rust");//s comes into the scope

    takes_ownership(s); //s's value moves into the function takes_ownership
    let x = 5; //x comes into the scope

    makes_copy(x); //Because i32 implements the Copy trait;
    //x does not move into the function so x can be used afterward
} //here x goes out of scope then s, However, because s value was moved, nothing special occurs

fn takes_ownership(some_string: String){ //some_string comes into the scope
    println!("{some_string}");
} //here some_string goes out of the scope and 'drop' is called
//backing memory is freed

fn makes_copy(some_integer: i32){ //some_integer comes into the scope
    println!("{some_integer}");
 //here some_integer goes out of scope nothing special happens

}
//if one tries to use s after invocation of takes_ownership(), Rust would throw a compile-time error