enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:#?}", self)
    }
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let localhost = IpAddressKind::V4(127, 0, 0, 0);

    let message = Message::Move { x: 1, y: 2 };
    message.print();

    // NOTE: Option enum

    enum Option<T> {
        Some(T),
        None,
    };

    let x = 5;
    let y = Some(5);

    let sum = x + y.unwrap_or(0);
    let result = value_in_cents(Coin::Quarter(UsState::Arizona));



    let five = Some(6);
    let seven = plus_one(five);
    let none = plus_one(None);


    let some = Some(1);

    // NOTE: using of if let syntax
    if let Some(3) = some {
        println!("Three")
    }

}

fn route(ip_kind: IpAddressKind) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona, 
    Califrnia
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    };
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}