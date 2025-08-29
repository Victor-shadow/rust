// Define a trait named `Summary`
// Traits are like interfaces: they define behavior that types can implement
pub trait Summary {
    // This method must be implemented by any type that wants to customize the author summary
    fn summarize_author(&self) -> String;

    // This method has a default implementation
    // If a type doesn't override it, this version will be used
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Define a struct representing a social media post
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// Implement the `Summary` trait for `SocialPost`
impl Summary for SocialPost {
    // Provide a custom implementation for `summarize_author`
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // We don't implement `summarize`, so the default from the trait is used
}

// Define another struct representing a news article
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement the `Summary` trait for `NewsArticle`
impl Summary for NewsArticle {
    // Provide a custom implementation for `summarize_author`
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // Override the default `summarize` method with a custom one
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    // Create a SocialPost instance
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    // Uses the default `summarize()` method from the trait
    println!("1 new post: {}", post.summarize());

    // Create a NewsArticle instance
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    // Uses the custom `summarize()` method defined in the `impl`
    println!("New article available! {}", article.summarize());
}