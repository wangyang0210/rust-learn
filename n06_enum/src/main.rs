
#![allow(unused)]

// base define
// fn main() {
//     #[derive(Debug)]
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
// 
//     fn route(ip_kind: IpAddrKind) {}
//     
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     println!("four: {:?}, six: {:?}", four, six);
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }


// struct IP address
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
// 
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
// 
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
// 
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//             V4(String),
//             V6(String),
//         }
// 
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("home: {:?}, loopback: {:?}", home, loopback)
// }

// fn main() {
//     #[derive(Debug)]
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
// 
//     let home = IpAddr::V4(127, 0, 0, 1);
// 
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("home: {:?}, loopback: {:?}", home, loopback)
// }

// fn main() {
//     struct Ipv4Addr {
//         // --snip--
//     }
// 
//     struct Ipv6Addr {
//         // --snip--
//     }
// 
//     enum IpAddr {
//         V4(Ipv4Addr),
//         V6(Ipv6Addr),
//     }
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }


// fn main() {
//     struct QuitMessage; // 类单元结构体
//     struct MoveMessage {
//         x: i32,
//         y: i32,
//     }
//     struct WriteMessage(String); // 元组结构体
//     struct ChangeColorMessage(i32, i32, i32); // 元组结构体
// }


// fn main() { 
//     #[derive(Debug)]
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }
// 
//     impl Message {
//         fn call(&self) {
//             // method body would be defined here
//             println!("self: {:?}", self)
//         }
//     }
// 
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');
// 
//     let absent_number: Option<i32> = None;
//     println!("some_number: {:?}, some_char: {:?}, absent_number: {:?}", some_number, some_char, absent_number)
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
// 
//     let sum = x + y;
// }


// match
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// 
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// 
// fn main() {
//     
// }


// match bind to value
// 
// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
// 
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// 
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
// 
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }


// match Option<T>
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             // Some(i) => Some(i + 1),
//         }
//     }
// 
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("five: {:?}, six: {:?}, none: {:?}", five, six, none)
// }

// match catch-all
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
// 
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}
// }


// match _

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
// 
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}
// }


// match tupe
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}