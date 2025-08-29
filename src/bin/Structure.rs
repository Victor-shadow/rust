struct Employee{
    name:String,
    company:String,
    age:u32
}
fn main(){
 let mut output = Employee{
     company:String::from("Amazon"),
     name:String::from("Google"),
     age:50
 };
    //to modify a struct, the instance variable must be marked as mutable
    output.age = 40;
    println!("Name is: {}, company is: {}, age is: {}", output.name, output.company, output.age);
}