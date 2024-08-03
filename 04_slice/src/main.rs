// fn main() {
//     println!("Hello, world!");
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }


fn main() {
    let s = "hello";
    let _c = s;
    let b = s;
    println!("{}", b)
}