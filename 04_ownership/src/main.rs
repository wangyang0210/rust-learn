// fn main() {
//     // let s = String::from("hello");
//     let mut s = String::from("hello");
//
//     s.push_str(", world!"); // push_str() appends a literal to a String
//
//     println!("{s}"); // This will print `hello, world!`
// }


//// move
// fn main() {
//     let x = 5;
//     let y = x;
//     println!("x = {}, y = {}", x, y)
// }

//// move string
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // println!("{}, world!", s2);
//     println!("{}, world!", s1);
// }

//
// fn main() {
//     let x = 5;
//     let y = x;
//
//     println!("x = {}, y = {}", x, y);
// }

//// function
// fn main() {
//
//     let s = String::from("hello");  // s 进入作用域
//
//     println!("s作用域:{}", s);
//
//     takes_ownership(s);             // s 的值移动到函数里 ...
//     // ... 所以到这里不再有效
//
//     // println!("s离开了作用域{}", s);
//
//     let x = 5;                      // x 进入作用域
//
//     makes_copy(x);                  // x 应该移动函数里，
//     // 但 i32 是 Copy 的，所以在后面可继续使用 x
//
// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// // 所以不会有特殊操作
//
// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     // println!("{}", s); // 不能够找到s的作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
//
// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

////  return value and scope
// fn main() {
//     let s1 = gives_ownership();     // gives_ownership 将返回值移给 s1
//     println!("{}", s1);
//     let s2 = String::from("hello");     // s2 进入作用域
//
//     let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3
//     println!("{}", s3);
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// // 所以什么也不会发生。s1 移出作用域并被丢弃
//
// fn gives_ownership() -> String {     // gives_ownership 将返回值移动给
//     // 调用它的函数
//     let some_string = String::from("yours"); // some_string 进入作用域
//     some_string                              // 返回 some_string 并移出给调用的函数
// }
//
// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
//     a_string  // 返回 a_string 并移出给调用的函数
// }


//// tuple
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}