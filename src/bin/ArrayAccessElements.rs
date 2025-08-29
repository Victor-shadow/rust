fn main(){
    let array = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let one = array[0];
    let two = array[8];
    let three = [100; 10];
    let four= ["Java", "Kotlin", "Scala", "Rust", "Python", "Javascript", "Haskell", "Elixir", "C#", "C++"];

    //loop through the array to print the elements
    for item in four{
        println!("{}", item)
    }
    println!("The value of the element at index zero is {}", one);
    println!("The value of the element at index eight is {}", two);
    println!("The value of three is {:?}", three);

}