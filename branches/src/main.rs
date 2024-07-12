// fn main() {
//     // let number = 6;
//     let number = 4;
//     // if number > 5 {
//     //     println!("Number is greater than 5");
//     // } else {
//     //     println!("Number is less than or equal to 5");
//     // }
//     // if number {
//     //     println!("Number is greater than 5")
//     // }
//
//     // if number {
//     //     println!("Number is greater than 5")
//     // }
//
//     // if number != 0 {
//     //     println!("number was something other than zero");
//     // }
//
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// use if in let
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {}", number);
// }

//
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
