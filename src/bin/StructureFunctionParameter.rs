struct Employee{
    name:String,
    company:String,
    age: u32
}
fn main(){
 //initializes a structure
    let display = Employee{
        company:String::from("Netflix"),
        name:String::from("Google"),
        age:50
    };

    let output = Employee{
        company:String::from("Amazon"),
        name:String::from("Google"),
        age:40
    };
    //pass display and output to result
    result(display);
    result(output);
}
//fetch values of specific structure fields using the operator and print it to the console
fn result(output:Employee){
    println!("Name is: {} Company is: {} Age is: {}", output.name, output.company, output.age);
}