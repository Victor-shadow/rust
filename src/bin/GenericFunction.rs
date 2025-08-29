use std::fmt::Display; //use of Display Trait in Scope
//Display is used for formatting types into human-readable strings

fn main(){
    print_pro(10u8);
    print_pro(20u16);
    print_pro("Rust book");

}

fn print_pro<T:Display>(t:T){ //t is a placeholder for any type
    //syntax <Placeholder:Trait>
    println!("Inside print_pro generic function");
    println!("{}", t);
}