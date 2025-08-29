
fn main() {
    //initialize structure
    let emp1 = Employee{
        company:String::from("Rust rover Projects"),
        name:String::from("Rust"),
        age:50
    };

    let emp2 = Employee{
        company:String::from("Rust rover Projects"),
        name:String::from("Rust"),
        age:30
    };
    let elder = who_is_elder(emp1, emp2);
    println!("elder is");
    //print details of the elder employee
    display(elder);
}
//accepts instances of employee and compares their age
fn who_is_elder(emp1:Employee, emp2:Employee) -> Employee{
    if emp1.age > emp2.age {
        return emp1;
    } else {
        return emp2;
    }
}
//display  name, company and age of the employee
fn display(emp:Employee){
    println!("Name is :{} company is {} age is {} ", emp.name, emp.company, emp.age);
}

struct Employee{
    name:String,
    company:String,
    age: u32
}