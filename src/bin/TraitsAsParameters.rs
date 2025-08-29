

pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {}...", self.summarize_author())
    }
}

//define a Social Post Struct
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

//implement the trait for Summary
impl Summary for SocialPost{
    //provide a custom implementation for the method
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    //if the method is not implemented the default method from the trait is used
}

//define news article struct
pub struct Article{
    pub content: String,
    pub author: String,
    pub location: String,
    pub headline: String,
}

//implement the trait for summary
impl Summary for Article{
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    //override the default trait method
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) {}", self.author, self.headline, self.location, self.content)
    }
}
fn main(){
    //create a social post instance
    let post = SocialPost {
        username: String::from("Rust book"),
        content: String::from("Rust is a statically typed programming language"),
        reply: false,
        repost: false,
    };

    //use the default 'summarize method from the trait'
    println!("1 new post: {}", post.summarize());

    let article = Article{
        headline: String::from("Developer of Rust"),
        content: String::from("Rust is a systems programming language"),
        author: String::from("Steve Klabnik"),
        location: String::from("USA"),
    };

    println!("1 new article {}", article.summarize());

}