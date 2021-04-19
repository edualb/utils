// Structs

fn main() {
    println!("Hello, world!");
}

// Example:

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// Instantiating Struct
//
fn instantiating_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        ..user1
    };

    // user2 is mutable, while user1 not.
    user2.email = user1.email;
}

// Building Struct
//
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple Struct
//
struct Color(i32, i32, i32);
fn tuple_struct() {
    let black = Color(0, 0, 0);
}

// Ownership od struct Data