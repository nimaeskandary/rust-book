// 0n 6.3

fn main() {
    let home = IpAddr::V4(Ipv4Addr {
        value: (127, 0, 0, 1),
    });
    let loopback = IpAddr::V6(Ipv6Addr {
        value: String::from("::1"),
    });

    let m = Message::Write(String::from("hello"));
    m.call();

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

struct Ipv4Addr {
    value: (u8, u8, u8, u8),
}

struct Ipv6Addr {
    value: String,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
