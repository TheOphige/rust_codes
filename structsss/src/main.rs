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

fn users() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    let user3 = build_user(String::from("thisemail@example.com"),
     String::from("thisusername123"));

}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


// Unit-Like Structs
struct AlwaysEqual;

fn unit_like_structs() {
    let subject = AlwaysEqual;
}

fn main() {
    users();
    tuple_structs();
    unit_like_structs();
}