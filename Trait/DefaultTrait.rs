use std::iter::Sum;

trait Summary {
    fn Summarize(&self) -> String {
        String::from("Read more ...")
    }

    fn author(&self) -> String;
}

struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn Summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }

    fn author(&self) -> String {
        format!("{}", self.username)
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn author(&self) -> String {
        format!("Article from {}", self.location)
    }
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news {} by {}", item.Summarize(), item.author());
}

fn main() {
    let tweet = Tweet {
        username: String::from("Rustaceans"),
        content: String::from("Learning Rust trait Implementation today"),
    };

    let article = NewsArticle {
        headline: String::from("Rust is the futureee"),
        location: String::from("Dahak"),
    };

    println!("-- Tweet (Custom Summary)");
    notify(&tweet);

    println!("\n ----------- News Article (Default summary -------");
    notify(&article);
}
