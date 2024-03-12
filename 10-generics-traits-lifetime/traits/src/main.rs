use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// NOTE: trait allow us to define shared method across different types
pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_v2(&self) -> String {
        String::from("default impl fro trait ...")
    }
}

// NOTE: we implement summary trait for news article
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_v2(&self) -> String {
        format!("Not default iml of str")
    }
}
fn main() {
    let new_article = NewsArticle {
        author: String::from("Killian Myrfy"),
        headline: String::from("We love killing people!"),
        content: String::from("IDK"),
    };

    let summary = new_article.summarize();
    println!("{}", summary);

    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: true,
    };

    println!("{}", tweet.summarize());

    println!(
        "Summarize v1 in news article - {}",
        new_article.summarize_v2()
    );
    println!("Summarize v1 in tweet - {}", tweet.summarize_v2());

    notify(&tweet);

    use_to_string();
}

// NOTE: traits as params, so the receiver should impl the Summary trait
// Also syntax in a bottom is sugar for trait bounds
pub fn notify(receiver: &impl Summary) {
    println!("Important news -> {}", receiver.summarize());
}

// NOTE: this is same as above also called trait bounds
pub fn notify_v2<T: Summary>(receiver: &T) {
    println!("Important news v2 -> {}", receiver.summarize());
}

pub fn notify_v3<T: Summary + Display>(item1: &T, item2: &T) {
    todo!()
}

fn some_fn_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_fn_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Debug + Copy,
{
    0
}


pub struct CustomToString<T> {
   pub data: T
}

impl<T: Display> ToString for CustomToString<T> {
    fn to_string(&self) -> String {
        format!("Test - {}", self.data)
    }
}


fn use_to_string() {
    let custom = CustomToString {
        data: false
    };

    println!("{}", custom.to_string());
}