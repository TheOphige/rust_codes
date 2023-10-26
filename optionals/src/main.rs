#![deny(clippy::all)]

// unwrapping options safely
// fn main() {
//     let name: Option<&str>= None;
//     match name {
//         Some(name) => println!("Hello, {}!", name),
//         None => println!("There is no name"),
//     };
// }

// unwrapping options unsafely
// fn main() {
//     let name: Option<&str>= Some("John Doe");
//     let unwrapped_name = name.expect("name was not provided");
//     println!("Name is {}", unwrapped_name);
// }

// fn main() {
//     let name: Option<&str>= Some("Peter");
//     let unwrapped_name = name.unwrap();
//     println!("Name is {}", unwrapped_name);
// }

// fn main() {
//     let age: Option<i32>= None;
//     // the default value of i32 is 0
//     let unwrapped_age = age.unwrap_or_default();
//     println!("{}", unwrapped_age);
// }

fn main() {
    let age: Option<i32>= Some(20);
    // the default value of i32 is 0
    let age_multiplied_by_2 = age.map(
        |age| age * 2
    );
    println!("{}", age_multiplied_by_2.unwrap_or_default());
}

// fn main() {
//     let name: Option<&str>= None;
//     let unwrapped_name = name.unwrap_or("John Doe");
//     println!("Name is {}", unwrapped_name);
// }

// fn main() {
//     let name: Option<&str>= None;
//     let unwrapped_name = name.unwrap_or_else(|| {
//         // do some work
//         "John Doe"
//     });
//     println!("Name is {}", unwrapped_name);
// }

// fn main() {
//     let mut age: Option<i8>= Some(20);
//     match age.as_mut() {
//         Some(age) => *age += 10,
//         None => {},
//     };
//     println!("Age is {}", age.unwrap());
// }

// to unwrap multiple optionals at once
// fn main() {
//     let age1: Option<i8>= Some(20);
//     let age2: Option<i8>= Some(30);
//     let age3: Option<i8>= Some(40);
//     if let (Some(age_1), Some(age_2), Some(age_3)) = (age1, age2, age3) {
//         println!("{}, {}, {}", age_1, age_2, age_3);
//     }
// }

// fn main() {
//     let name: Option<&str>= None;
//     if name.is_some() {
//         println!("There is a value");
//     } else {
//         println!("There is no value");
//     }
// }