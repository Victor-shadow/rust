fn main(){
    //create an instance of struct
    let b1 = Book {
        id: 1000,
        name: "Rust in Action"
    };

    b1.print();
}
//declare a structure
struct Book {
    name:&'static str,
    id: u32
}

//declare a trait
trait Printable{
    fn print(&self);
}
//implement the trait
impl Printable for Book {
    fn print(&self) {
        println!("Print book with id:{} and name{}", self.id, self.name);
    }
}