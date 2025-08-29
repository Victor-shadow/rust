fn main(){

    let arr:[i32; 5] = [10, 20, 30, 40, 50];
    println!("array is{:?}, ", arr);
    println!("arr size is {}, ", arr.len());

    for value in arr.iter(){
        println!("value is :{}", value);
    }
}