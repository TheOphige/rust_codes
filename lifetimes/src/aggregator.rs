pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub struct Thread {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub rethread: bool,
}

// using default summary
impl Summary for Thread {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// This parameter accepts any type that implements the specified trait. e.g. Tweets, NewsArticle
// In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// The above (impl Trait) is a simple version of the (trait bound) below
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// or using where
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!()
// }
