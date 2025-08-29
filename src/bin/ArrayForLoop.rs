fn main(){
    //Array Iteration to print all the elements and their index
    let arr = [100, 200, 300, 400];
    println!("array is {:?}", arr);
    print!("array size is :{}", arr.len());

    for index in 0..arr.len(){
        println!("index is: {} & value is: {}", index, arr[index]);
    }
}