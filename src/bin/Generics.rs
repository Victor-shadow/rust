//a generic function named largest
//<T: PartialOrd> - this generic type works with any value of T and can be used for comparison
//list: &[T] -the function takes a reference to  a slice of items in a list
//&T = it returns reference to the largest items in a list
fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main(){
    let number_list = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char = vec!['y', 'm', 'a', 'l' ];
    let result = largest(&char);

    print!("The largest char is {result}");


}