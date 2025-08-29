fn main() {
    //the value of an identifier prefixed with the const keyword is defined at compile time
    const N: usize = 20; //usize is pointer sized(used for indexing and memory sizes) it matches the size of a  pointer in the system

    //pointer sized
    let arr = [0; N];
    println!("{}", arr[10]);
}