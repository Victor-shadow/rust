fn main(){
  //an array can be passed as a value or by reference to functions
    let arr = [100.0, 200.0, 300.0, 400.0, 500.0];
    update(arr); //pass the array as a parameter to the update method

    println!("Inside main function {:?},", arr);
}
fn update(mut arr: [f64; 5]){
    for i in 0..3{ //update array elements upto index 3
        arr[i] = 0.0;
    }
    println!("Inside update method {:?},", arr);
}