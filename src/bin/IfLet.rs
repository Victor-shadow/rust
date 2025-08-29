fn main(){
    //a variable assigned value of 10
    let variable = Some(10);
    //if let operator
    let output = if let Some(10) = variable{
        11
    } else if Some(20) == variable{
        22
    } else if let Some(30) = variable{
        33
    } else {
        -1
    };

    println!("{}", output)
}