// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
// 
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }
// 
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
// 
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }
// 
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
// 
//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");
//     assert_eq!(*result, 100);
// 
//     let char_list = vec!['y', 'm', 'a', 'q'];
// 
//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
//     assert_eq!(*result, 'y');
// }

//// error
// fn largest<T>(list: &[T]) -> &T {
//// fix
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
// 
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
// 
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
// 
//     let char_list = vec!['y', 'm', 'a', 'q'];
// 
//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }


// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     println!("{:?}", integer);
//     println!("{:?}", float);
// }


// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}