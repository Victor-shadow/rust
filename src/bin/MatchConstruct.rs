
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin : Coin, coin0: Coin) -> u8 {
    match coin { //match expression
        //the type of coin is the enum Coin
        //the match arm; an arm has two parts: a pattern and code
        Coin::Penny => 1, //(pattern)Value:: Value => 1(code)
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}!");
            25
            //when the match expression executes, it compares the resultant value against the pattern of each arm in order
            //If a pattern matches the value, the code associated with that pattern gets executed
            //the code associated with each arm is an expression, and the resultant value of the expression is the value that gets returned from the entire match expression
        }
    }
}

fn main(){
    value_in_cents(Coin::Quarter(UsState::Alaska), Coin::Penny);
    println!("The value in cents for the coins are: {}", value_in_cents(Coin::Dime, Coin::Penny));

}
