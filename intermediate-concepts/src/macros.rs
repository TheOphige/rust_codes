
use std::collections::HashMap;

// How Macros are created
// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push(($x));
//             )*
//             temp_vec
//         }
//     };
// }

// How it works
// Make this macro available elsewhere
#[macro_export]
// Establish the name of the macro
macro_rules! vec {
    // Match any Rust expression [$x:expr]
    // Allows one or more of them [,*]
    ( $( $x:expr ),* ) => {
        {
            // Create a temporary empty vector
            let mut temp_vec = Vec::new();
            // Generate this command for all expressions
            $(
                temp_vec.push(($x));
            )*
            temp_vec
        }
    };
}


// Example
// let x = vec![1, 2];
// now works as
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec
// }

#[macro_export]
macro_rules! make_map {
    ( $k:expr, $v:expr ) => {
        {
            println!("Key: {}", $k);
            println!("Key: {}", $v);
            let mut map = HashMap::new();
            map.insert($k, $v);
            map
        }
    };
}

#[macro_export]
macro_rules! make_map_2 {
    ( $( (  $k:expr, $v:expr ) ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                println!("Key: {}", $k);
                println!("Key: {}", $v);
                map.insert($k, $v);
            )*
            map
        }
    };
}



fn main() {
    let int_map: HashMap<i32, i32> = make_map!(1, 3);
    println!("{:#?}", int_map);
    let str_map: HashMap<&str, &str> = make_map!("green", "go");
    println!("{:#?}", str_map);

    let int_map_2: HashMap<i32, i32> = make_map_2![
        (1, 3),
        (2, -1),
        (3, 5)
    ];
    println!("{:#?}", int_map_2);
    let str_map_2: HashMap<&str, &str> = make_map_2![
        ("green", "go"),
        ("yellow", "slow"),
        ("red", "stop")
    ];
    println!("{:#?}", str_map_2);
}
