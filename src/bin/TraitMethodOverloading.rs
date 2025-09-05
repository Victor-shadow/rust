//two traits Wizard and Pilot have the same method fly
//The traits are both implemented on a Struct Human that already has a method fly implementing on it

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is the captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("*waving arms furiously");
    }
}

fn main(){
    //When fly is called on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type
    //Running this code prints *waving arms furiously showing that Rust called the fly method implemented
    //on the Human instance
    //To call the fly method from either Pilot or Wizard trait one needs to use more explicit syntax to specify which fly methods is to be used

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);

    //specify the trait name before the method name clarifies to Rust which implementation of fly
    //is to be called
    //One can also use Human::fly(&person) which is similar to person.fly but this is longer to write if we do not need to disambiguate


}