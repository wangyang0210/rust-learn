
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
