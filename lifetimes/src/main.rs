// lifetimes Traits Generics
#![deny(clippy::all)]

// fn get_full_name() -> &'static str {
//     "John Doe"
// }

// fn get_random_name<'l1>(a: &'l1 str, b: &'l1 str) -> &'l1 str {
//     a
// }

// fn main() {
//     let name = get_random_name("John", "Doe");
//     println!("{}", name);
// }

// struct Person<'a> {
//     first_name: &'a str,
//     last_name: &'a str,
// }
// impl<'a> Person<'a> {
//     fn first_char_of_first_name(&self) -> &str {
//         &self.first_name[0..1]
//     }
//     fn get_full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// fn get_first_name(full_name: &str) -> &str {
//     full_name
// }

// enum Animal<'a> {
//     Dog {name: &'a str},
// }                   



// The first rule is that the compiler assigns a 
// lifetime parameter to each parameter that’s a 
// reference. In other words, a function with one 
// parameter gets one lifetime parameter: 
// fn foo<'a>(x: &'a i32); a function with two parameters 
// gets two separate lifetime parameters: 
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is that, if there is exactly one input 
// lifetime parameter, that lifetime is assigned to all 
// output lifetime parameters: 
// fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input 
// lifetime parameters, but one of them is &self or 
// &mut self because this is a method, the lifetime 
// of self is assigned to all output lifetime parameters. 
// This third rule makes methods much nicer to read and 
// write because fewer symbols are necessary.


// traits

// use std::fmt;

// struct Person {
//     first_name: String,
//     last_name: String,
//     age: u8,
// }

// // a trait for println! while using the display syntax {}
// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f, "{} {} is {} years old", 
//             self.first_name, self.last_name, self.age
//         )
//     }
// }

// trait HasFullName {
//     fn full_name(&self) -> String;
// }


// impl HasFullName for Person {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// trait CanInitializeWithFullName {
//     fn new(full_name: &str) -> Self;
// }

// impl CanInitializeWithFullName for Person {
//     fn new(full_name: &str) -> Self {
//         let parts: Vec<&str> = full_name.split(' ').collect();
//         Person {
//             first_name: parts[0].to_string(),
//             last_name: parts[1].to_string(),
//             age: 30,
//         }
//     }
// }

// fn print_full_name_and_age(value: &impl HasFullName) {
//     println!("{}", value.full_name())
// }

// fn print_details<T>(value: &T) where
//     T: HasFullName + CanRun,
// {
//     println!("{}", value.full_name())
// }

// trait CanRun {
//     fn run(&self);
// }

// impl CanRun for Person {
//     fn run(&self) {

//     }
// }

// fn main() {
//     let person = Person::new("John Doe");
//     println!("{}", person);
//     println!("{}", Person::full_name(&person));
//     print_full_name_and_age(&person);
//     print_details(&person);
// }

// Generics

// the function largest is generic over some type T. 
// This function has one parameter named list, which is 
// a slice of values of type T. The largest function will 
// return a reference to a value of the same type T.
// To enable comparisons, the standard library has the 
// std::cmp::PartialOrd trait that you can implement on 
// types


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


struct Point<T, U> {
    x: T,
    y: U,
}
// a trait for println! while using the display syntax {}
// use std::fmt;
// impl<T: fmt::Display, U: fmt::Display> fmt::Display for Point<T, U> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f, "x is {}, y is {}.", 
//             self.x, self.y
//         )
//     }
// }

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//concrete type f32, meaning 
// we don’t declare any types after impl.
impl Point<f32, f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// importing traits & types from aggregator
mod aggregator;
use aggregator::{Summary, Tweet, NewsArticle, Thread, notify};


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let _both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
    //println!("{}", both_float);
    println!("x is {:?}, y is {:?}.", both_float.x, both_float.y);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {} & p.y = {}", p.x(), p.y());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let newsarticle = NewsArticle {
        headline: String::from("Nigeria is the best country in the world"),
        location: String::from("oyo"),
        author: String::from("jamiu_olushola"),
        content: String::from(
            "definitely people in that country are technologically inclined",
        ),
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let thread = Thread {
        username: String::from("juju_boy"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        rethread: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new thread: {}", thread.summarize());
    println!("1 news article: {}", newsarticle.summarize());
    println!("New article available! {}", article.summarize());
    notify(&newsarticle)
}

