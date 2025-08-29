

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping Customer Smart Pointer with data `{}`!", self.data);
    }
}

fn main(){
    let c = CustomSmartPointer{
        data: String::from("Rust language"),
    };

    let d = CustomSmartPointer{
        data: String::from("Old C++"),

    };
    println!("Customer pointer data created")

}
//the Drop Trait is included in the prelude, so it does not need to be brought into the scope
//The Drop trait is implemented on CustomSmartPointer and provides an implementation of the drop method
//that calls println!
//the body of the drop method is where logic to be run is placed when an instance of a type goes out of scope
//Rust automatically calls the drop method when instances went out of scope calling the code that was specified
//Variables are dropped in the reverse order of their creation, so d is dropped before c