
#[derive(Debug)]
 enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", &self)
        
    }
}

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn rolll() {
    // you can create a catch all variable for remaining values of a 
    // match pattern since match are exhaustive 
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // or if you don't roll 3 or 7 and you don't need the value 
    // and you just want to reroll, u can use _
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // or if you want nothing to happen if you don't roll 3 or 7, 
    // you can use an empty tuple
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}


// let config_max = Some(3u8);
// match config_max {
//     Some(max) => println!("The maximum is configured to be {}", max),
//     _ => (),
// }


// intead of using _ => (), we use if let
// if pattern matches, let this = that
// if let helps avoid verbose(wordy) code
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}


// let coin = Coin::Penny;
// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//     _ => count += 1,
// }

// else can be used for performing actions for all other unmatched patterns
let coin = Coin::Penny;
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}




fn main() {

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));
    println!("{:?}, {:?}", home2, loopback2);

    let m = Message::Write(String::from("hello"));
    m.call()
}


