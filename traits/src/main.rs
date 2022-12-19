use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_format(&self) -> String {
        format!("{} (Read more...)", self.summarize())
    }
}

pub trait Display {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
impl<T: Display> ToString for T {

    // code
}*/

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanly Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburgh Penguins once again are the best hockey team in the NHL",
        ),
    };

    println!("New article available! {}", article.summarize_default());
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_type<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_multiple(item: &(impl Summary + Display)) {}

fn notify_multiple_type<T: Summary + Display>(item: &T) {}

// This gets messy with multiple generics, so we can use the where clause:

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
// you cannot return a Summary if you return different types.


// We can also conditionally implement a trait

