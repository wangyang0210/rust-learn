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


// fn main() {
//     // let s = "hello";
//     // let _c = s;
//     // let b = s;
//     // println!("{}", b)
//     let s = String::from("hello world");
//
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{}, {}", hello, world)
// }

// fn main() {
//     let s = String::from("hello");
//
//     let slice1 = &s[0..2];
//     let slice2 = &s[..2];
//     println!("{}", slice1);
//     println!("{}", slice2);
// }


// fn main() {
//     let s = String::from("hello");
//
//     let len = s.len();
//
//     let slice1 = &s[3..len];
//     let slice2 = &s[3..];
//     println!("{}", slice1);
//     println!("{}", slice2)
// }


// fn main() {
//     let s = String::from("hello");
//
//     let len = s.len();
//
//     let slice1 = &s[..len];
//     let slice2 = &s[..];
//     println!("{}", slice1);
//     println!("{}", slice2)
// }


// fn main() {
//     let my_string = String::from("hello world");
//     let word = first_word(&my_string);
//     println!("{}", word)
// }
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }


// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     s.clear(); // error!
//
//     println!("the first word is: {word}");
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }
//
// fn main() {
//     let my_string = String::from("hello world");
//
//     // `first_word` works on slices of `String`s, whether partial or whole
//     // `first_word` 接受 `String` 的切片，无论是部分还是全部
//     let word1 = first_word(&my_string[0..6]);
//     let word2 = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // `first_word` 也接受 `String` 的引用，
//     // to whole slices of `String`s
//     // 这等同于 `String` 的全部切片
//     let word3 = first_word(&my_string);
//     println!("1{}", word1);
//     println!("2{}", word2);
//     println!("3{}", word3);
//
//     let my_string_literal = "hello world";
//
//     // `first_word` works on slices of string literals, whether partial or whole
//     // `first_word` 接受字符串字面量的切片，无论是部分还是全部
//     let word4 = first_word(&my_string_literal[0..6]);
//     let word5 = first_word(&my_string_literal[..]);
//     println!("4{}", word4);
//     println!("5{}", word5);
//
//     // Because string literals *are* string slices already,
//     // 因为字符串字面值**就是**字符串 slice，
//     // this works too, without the slice syntax!
//     // 这样写也可以，即不使用 slice 语法！
//     let word6 = first_word(my_string_literal);
//     println!("6{}", word6);
// }
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }


fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a has {} elements", a.len());
    let slice = &a[1..3];
    println!("{:?}", slice);
}
