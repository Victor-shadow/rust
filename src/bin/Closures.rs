#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color{
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
//The store defined in main has two blue shirts and one red  shirt remaining to distribute
//for a limited edition promotion
//We call the giveaway function for a user with preference for a red shirt and a user with no preference
fn main(){
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway
    );

    let user_pref2 = Some(ShirtColor::Blue);
    let  giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
//In the giveaway method, we get the user preference as a parameter of type Option<ShirtColor> and
//call the unwrap_or_else method on user_preference
//The unwrap_or_else method on Option<T> is defined by the Standard library. It takes one argument a closure
//without any arguments that returns a value T(same type stored in a Some variant Of the Option<T> in this case Shirt Color)
//If the Option<T> is Some variant, unwrap or else returns the value from within Some
//If the Option<>T is a None Variant, unwrap_or_else calls the closure and returns the value  returned by the closure
//We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else
//This is a closure that takes no parameters itself(if closures had parameters, they would appear between two vertical bars)
//The body of the closure calls the self.most_stocked(). We define the closure here and  the implementation of the unwrap or else will evaluate the closure later if
//the result is needed
//We passed a closure that calls self.most_stocked() on the instance of Inventory
//The closure captures the immutable reference to the self Inventory instance and passes it with the code
//with the code we specify the unwrap_or_else method