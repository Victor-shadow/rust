fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main(){

    let five = Some(5);
    let six = plus_one(five); //when this method is invoked, the variable x in the body of plus_one will have a value Some(5)
    let none = plus_one(None);

}