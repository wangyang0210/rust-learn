// fn main() {
//     use std::collections::HashMap;
// 
//     let mut scores = HashMap::new();
// 
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// }

// fn main() {
//     use std::collections::HashMap;
// 
//     let mut scores = HashMap::new();
// 
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// 
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
// }

// fn main() {
//     use std::collections::HashMap;
// 
//     let mut scores = HashMap::new();
// 
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// 
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

// fn main() {
//     use std::collections::HashMap;
// 
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");
// 
//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
// }

// fn main() {
//     use std::collections::HashMap;
// 
//     let mut scores = HashMap::new();
// 
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);
// 
//     println!("{scores:?}");
// }

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}