
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


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    
}