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

mod front_of_house {
     mod hosting {
         fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn main() {
    eat_at_restaurant();
}