fn main(){
    //an array of strings
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    //an array that uses type annotation with a fixed size of elements
    //i32: The type of element
    //10: The number of elements stored in the array
    let numbers:[i32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{:?}", months);
    println!("{:?}", numbers);

}