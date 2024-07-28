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

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}