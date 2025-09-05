//Creation of a trait for an animal shelter that wants to name all baby dogs spot
//An Animal trait with an associated non method function baby_name
//The Animal trait  is implemented for the Struct Dog, on which one also provides an associated non-method function
//baby dog directly

trait Animal {
    fn baby_name() -> String;
}
struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // Call the trait method explicitly using fully qualified syntax
    // println!("A baby dog is called a {}", Dog ::baby_name());
    //provide Rust with a type annotation within the Angle brackets, which indicates that one wants to call baby_name method from Animal trait as implemented on Dog
    //the Dog type is treated as an Animal for the function call

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
//We implement the code for naming all puppies. Spot in the baby_name associated function that is defined on Dog
//The Dog type also implements the trait Animal, which describe characteristics all animals have
//In main one calls the Dog::baby_name function, which calls the associated function defined on Dog directly

//NOTE: This gets a Compilation error: println!("A baby dog is called a {}", Animal::baby_name());
//this is because Animal::baby_name does not have a self parameter, and there could be other types that implement the Animal trait
//Rust is unable to figure out which  implementation of Animal::baby_name is needed