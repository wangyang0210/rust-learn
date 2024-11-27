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

// fn main() {
//     use std::collections::HashMap;
// 
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
// 
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
// 
//     println!("{scores:?}");
// }

// fn main() {
//     use std::collections::HashMap;
// 
//     let text = "hello world wonderful world";
// 
//     let mut map = HashMap::new();
// 
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
// 
//     println!("{map:?}");
// }

use std::collections::HashMap;

fn find_median_and_mode(mut nums: Vec<i32>) -> (f64, i32) {
    // 计算中位数
    nums.sort();
    let mid = nums.len() / 2;
    let median = if nums.len() % 2 == 0 {
        (nums[mid - 1] as f64 + nums[mid] as f64) / 2.0
    } else {
        nums[mid] as f64
    };

    // 计算众数
    let mut frequency_map = HashMap::new();
    for &num in &nums {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mode = frequency_map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap_or(0);

    (median, mode)
}

fn main() {
    let nums = vec![3, 1, 2, 2, 5, 2];
    let (median, mode) = find_median_and_mode(nums);
    println!("中位数: {}", median);
    println!("众数: {}", mode);
}
