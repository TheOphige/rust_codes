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
struct Points(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Points(0, 0, 0);
}


// Unit-Like Structs
struct AlwaysEqual;

fn unit_like_structs() {
    let subject = AlwaysEqual;
}

// point struct example
#[deny(clippy::all)]

#[derive(Debug)]
struct Point(f64, f64, f64);

impl Point {
    // method that change the struct value
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    // method that use the struct but does not change struct
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
    fn describe(&self) {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }
    // doesnt take a reference to self, just calls point
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}



fn main() {
    users();
    tuple_structs();
    unit_like_structs();

    let p = Point(1.0, 2.0, 3.0);
    p.describe();
    println!("{:?}", p);
    
    let mut point = Point(1.0, 2.0, 3.0);
    let twice = point.twice();
    println!("{:?}", twice);
    point.make_twice();
    println!("{:?}", point);

    let point = Point::zero;
    let point1 = Point::zero;
    let point2 = Point::zero;

}