#![allow(unused)]
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
// 
//         fn seat_at_table() {}
//     }
// 
//     mod serving {
//         fn take_order() {}
// 
//         fn serve_order() {}
// 
//         fn take_payment() {}
//     }
// }

// mod front_of_house {
//      pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
// 
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
// 
// fn deliver_order() {}
// 
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
// 
//     fn cook_order() {}
// }


// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
// 
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// 
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }


// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// 
//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// use crate::front_of_house::hosting;
// 
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }
// 
// fn main() {
//     eat_at_restaurant();
// }
// 
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// use crate::front_of_house::hosting;
// 
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// #![allow(unused)]
// fn main() {
//     use std::fmt;
//     use std::io;
// 
//     fn function1() -> fmt::Result {
//         // --snip--
//         Ok(())
//     }
// 
//     fn function2() -> io::Result<()> {
//         // --snip--
//         Ok(())
//     }
// }

// 
// #![allow(unused)]
// fn main() {
//     use std::fmt::Result;
//     use std::io::Result as IoResult;
// 
//     fn function1() -> Result {
//         // --snip--
//         Ok(())
//     }
// 
//     fn function2() -> IoResult<()> {
//         // --snip--
//         Ok(())
//     }
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// pub use crate::front_of_house::hosting;
// 
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// use rand::Rng;
// 
// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..101);
// }

#![allow(unused)]
fn main() {
    use std::collections::HashMap;
}