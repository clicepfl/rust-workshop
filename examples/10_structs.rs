#![allow(dead_code, unused_variables, clippy::all)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax
    let user2 = User {
        email: dbg!(String::from("another@example.com")),
        ..user1
    };
}

// Field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

impl User {
    // Method
    fn active(&self) -> bool {
        self.active
    }

    // Associated function
    fn new(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

// Multiple impl blocks
impl User {
    fn sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
}

// Tuple struct
struct Color(u8, u8, u8);

// Unit-like struct
struct AlwaysEqual;

// Type alias
type Kilometers = i32;

// Self type alias
impl User {
    fn new_user(email: String, username: String) -> Self {
        Self {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}
