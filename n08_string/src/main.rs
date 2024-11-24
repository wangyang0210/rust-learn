
// create a new string
// fn main() {
//     let mut s = String::new();
// }

// create a string from a string literal
// fn main() {
//     let data = "initial contents";
//
//     let s = data.to_string();
//     println!("sting s ===>: {s}");
//
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
//     println!("s=====>: {s}");
// }


// fn main() {
//     let s = String::from("initial contents");
//     println!("s=====>: {s}");
// }


//  use push_str
// fn main() {
//     let mut s = String::from("foo");
//     s.push_str("bar");
//     println!("s: {s}");
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {s2}");
// }

// fn main() {
//     let mut s = String::from("lo");
//     s.push('l');
//     println!("s: {s}");
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//     println!("s3:{s3}")
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//
//     let s = s1 + "-" + &s2 + "-" + &s3;
//     println!("s:{s}")
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
// 
//     let s = format!("{s1}-{s2}-{s3}");
//     println!("s:{s}")
// }

// fn main() {
//     let s1 = String::from("hello");
//     let h = s1[0];
// }

fn main() {
    let hello = "Здравствуйте";
    let answer = &hello[0];
}
