fn main(){
    let a = [10, 20, 30];
    //a[10]; //invokes a panic since the index is unreachable
    println!("{:?}", a);
}