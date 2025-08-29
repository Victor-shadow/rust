use std::fmt::Display;

// Function that returns the longer of two string slices, with an announcement
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    // Print the announcement using the Display trait
    println!("Announcement! {}", ann);

    // Return the longer string slice
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Shadow is learning Rust");
    let s2 = String::from("Rust is powerful");

    let result = longest_with_an_announcement(&s1, &s2, "Comparing strings...");
    println!("Longest string: {}", result);
}