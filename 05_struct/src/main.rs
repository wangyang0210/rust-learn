// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("hello"),
//         email: String::from("hello@easybe.org"),
//         sign_in_count: 1,
//     };
//
//     println!("{}", user1.username)
// }
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("{}", user1.email);
    println!("{}", user1.username);

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // println!("{}", user2.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.email)
}