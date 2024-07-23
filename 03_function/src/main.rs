// fn main() {
//     // println!("Hello, world!");
//     // author_function();
// }

// fn main() {
//     author_function("Tim", 10);
// }
//
// fn author_function(name: &str, age: i32) {
//     println!("author function name: {}, age:{}", name, age)
// }

// fn main() {
//     let x = (let y = 6);
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {}", y);
// }

// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {}", x);
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}


