#![deny(clippy::all)]


// create settings.json to help you automatically format your
// code on save

// rustup component add clippy
// cargo install cargo-watch
// cargo-watch -qc -x run -x clippy

// cargo run -q
// cargo run --bin gugu



// fn main() {
//     println!("Hello world!!!");
// }

// function
// fn say_hello_world(to_person: String) -> String {
//     format!("Hello, {}!", to_person)
// }
// fn main() {
//     let hello = say_hello_world(String::from("John"));
//     println!("{}", hello);
// }

// inline function
// fn main() {
//     let say_hello_to 
//         = |name: &str| format!("Hello, {}!", name);

//     println!("{}", say_hello_to("World"));
// }

// full name inline function
// fn main() {
//     let full_name 
//         = |first_name: &str, last_name: &str|
//             format!("{} {}!", first_name, last_name);

//     println!("{}", full_name("taiye", "kehinde"));
// }


// fn main() {
//     let multiply_by_2 = |x: i32| x * 2;
//     let ptr = multiply_by_2;
//     let result = ptr(10);
//     println!("{}", result);
// }

// fn main() {
//     let ask_for_age = || {
//         // ask the user for age
//         // calculate how old in 10 years
//         10
//     };
// }


fn process_name(name: &str, callback: fn(&str) -> String) -> String {
    callback(name)
}

fn main() {
    let say_hello_to = |name: &str| format!("Hello, {}!", name);
    println!("{}", process_name("World", say_hello_to));
}
