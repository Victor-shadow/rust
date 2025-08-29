fn main(){
    //a mutable array
    let mut data = [100, 200, 300, 400, 500, 600, 700, 800, 900];
    use_slice(&mut data[1..6]);
    println!("{:?}", data);
}

fn use_slice(slice: &mut [i32]){
    println!("the length of the slice is {:?}", slice.len());
    println!("{:?}", slice);
    slice[0] = 10; //replace 100 with 10
}