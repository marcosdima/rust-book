#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState, UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state1, state2) => {
            println!("State quarter from {state1:?} and {state2:?}!");
            25
        }
    }
}

fn describe_option(x: Option<i32>) {
    match x {
        None => println!("No value"),
        Some(i) if i > 0 => println!("Positive"),
        Some(i) if i < 0 => println!("Negative"),
        Some(_) => println!("Zero"),
    }
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska, UsState::Alabama));
    describe_option(Some(0));
}
