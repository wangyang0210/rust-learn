
// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];
    // let mut v = vec![1, 2, 3];
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // println!("push v,{:?}",v);
    
    // read vector 
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    
    // read element outside the range of existing elements.
    // let does_not_exist = &v[100];
    // println!("use Index read outside the range element: {}",does_not_exist);
//     let does_not_exist = v.get(100);
//     println!("use get read outside the range element: {:?}",does_not_exist);
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
// 
//     let first = &v[0];
// 
//     v.push(4);
// 
//     println!("The first element is: {first}");
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     for i in &v {
//         println!("{i}");
//     }
// }


// fn main() {
//     let mut v = vec![1, 2, 3];
//     for i in &mut v {
//         *i += 50;
//     }
// }

fn main() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row:{:?}", row);
}