// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");
//     change(&s);
// }
//
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//
//     println!("{}, {}, and {}", r1, r2, r3);
// }

// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}");
//     // variables r1 and r2 will not be used after this point
//
//     let r3 = &mut s; // no problem
//     println!("{r3}");
// }

// 悬垂引用

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn main() {
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}");
}
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}