
// 变量
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// 常量
// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
// }

// 遮蔽
// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is: {x}");
// }

// 遮蔽和可变变量
// fn main() {
//     let spaces = "  ";
//     let spaces = spaces.len();
//     println!("spaces: {spaces}")
// }

// fn main() {
//     let mut spaces = "  ";
//     spaces = spaces.len();
//     println!("spaces: {spaces}")
// }

// 标量类型和复合类型
// fn main() {
//     let guess = "42".parse().expect("Not a number!");
//     println!("guess: {guess}")
// }


// 浮点数
// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("x: {x}, y: {y}")
// }

// 数值运算
// fn main() {
//     let sum = 5 + 10;
//     let difference = 95.5 - 4.3;
//     let product = 4 * 30;
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3;
//     let remainder = 43 % 5;
//     println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, floored: {floored}, remainder: {remainder}")
// }

// 布尔类型
// fn main() {
//     let t = true;
//     let f: bool = false; // 显式类型标注
//     println!("t: {t}, f: {f}")
// }


// 字符类型
// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}")
// }

// 元组
// fn main() {
//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("x:{x}, y: {y}, z: {z}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
//     println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}")
// }


// 数组
// fn main() {
//     // let a = [1, 2, 3, 4, 5];
//     let a:[i32;6] = [1, 2, 3, 4, 5, 6];
//     // let a = [3; 5];
//     let first = a[0];
//     let second = a[1];
//     println!("a:{:?}", a);
//     println!("first: {first}, second: {second}")
// }

// 无效的数组访问