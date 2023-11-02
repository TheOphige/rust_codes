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

use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

// a trait for println! while using the display syntax {}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{} {} is {} years old", 
            self.first_name, self.last_name, self.age
        )
    }
}

trait HasFullName {
    fn full_name(&self) -> String;
}


impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 30,
        }
    }
}

fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name())
}

fn print_details<T>(value: &T) where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name())
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {

    }
}


fn main() {
    let person = Person::new("John Doe");
    println!("{}", person);
    println!("{}", Person::full_name(&person));
    print_full_name_and_age(&person);
    print_details(&person);
}