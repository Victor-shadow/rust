#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}
fn main(){
    let scale = 5;
    let rec1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rec1) ;//one can put dbg! around the expression (30 * scale) since it returns ownership of the
    //expression value, the width field will get the same value when we also do not have dbg!
    //for dbg! not to have ownership of output  a reference is used in the next call
    
}