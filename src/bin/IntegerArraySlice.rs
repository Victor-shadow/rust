fn main(){
  let data = [100, 200, 300, 400, 500];
    use_slice(&data[1..4]);
    //effectively borrow elements for a while
}
fn use_slice(data: &[i32]){
    println!("length of slice is {:?}", data.len());
    println!("{:?}", data);
}
//the main function declares an array with 5 elements
//it invokes the use_slice function and passes it to a slice of three elements(points to the data array)
//slices are passed as reference
//the use_slice function prints the value of the slice and its length