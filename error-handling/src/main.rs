// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");
// }

// Propergating Errors
// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// }

// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
// }

// #![allow(unused)]
// fn main() {
// use std::fs;
// use std::io;

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

// fn main() {
//     assert_eq!(
//         last_char_of_first_line("Hello, world\nHow are you today?"),
//         Some('d')
//     );

//     assert_eq!(last_char_of_first_line(""), None);
//     assert_eq!(last_char_of_first_line("\nhi"), None);
//     assert_eq!(last_char_of_first_line("hi"), None);
// }

// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// fn main() {
//     let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");
//     match value {
//         Ok(value) => println!("{}", value),
//         Err(error) => println!("{}", error),
//     }
// }

// if the main error is not neccesary
// fn main() {
//     let value: Result<&str, ()> = Err(());
//     match value {
//         Ok(value) => println!("{}", value),
//         Err(_) => println!("Some error occurred"),
//     }
// }

// fn get_user_name() -> Result<String, ()> {
//     Ok("John".to_string())
//     // or 
//     // Err(())
// }
// fn main() {
//     let user_name = get_user_name().expect("failed to get user name");
//     println!("Hello, {}!", user_name);
// }

// fn get_user_name() -> Result<String, ()> {
//     Ok("John".to_string())
//     // Err(())
// }
// fn main() {
//     let is_ok = get_user_name().is_ok();
//     let is_err = get_user_name().is_err();
//     println!("{} {}", is_ok, is_err)
// }

use std::fmt::format;

fn get_first_name() -> Result<String, ()> {
    Ok("John".to_string())
}

fn get_last_name() -> Result<String, ()> {
    // Ok("Doe".to_string())
    Err(())
}

fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}
fn main() {
    let full_name = get_full_name();
    // match full_name {
    //     Ok(name) => println!("Hello, {}!", name),
    //     Err(_) => println!("Error!"),
    // }

    let length 
        = full_name.map(|s| s.len()).unwrap_or_default();
    println!("{}", length);
}