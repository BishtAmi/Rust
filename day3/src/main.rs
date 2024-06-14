// structure
struct User {
    name: String,
    email: String,
    active: bool,
}

// tuple struct
struct Point(i32, i32, i32);
fn main() {
    let mut user = User {
        name: String::from("Amit Singh Bisht"),
        email: String::from("singhbishtamit2@gmail.com"),
        active: true,
    };
    user.name = String::from("Ami Bisht");
    println!("Username:{}", user.name);
    println!("Email:{}", user.email);
    println!("Status:{}", user.active);

    let mut user2 = User { ..user };
    user2.name = String::from("Sumi Bisht");
    println!("Username:{}", user2.name);
    println!("Email:{}", user2.email);
    println!("Status:{}", user2.active);

    // tuple struct
    let origin = Point(0, 2, 0);
    println!("Origin cordinates {}", origin.1);
}
