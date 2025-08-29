// Define a trait for sending messages
pub trait Messenger {
    fn send(&self, msg: &str);
}

// A concrete implementation of Messenger that stores messages
pub struct ConsoleMessenger;

impl Messenger for ConsoleMessenger {
    fn send(&self, msg: &str) {
        println!("Message sent: {}", msg);
    }
}

// Struct that tracks usage against a quota
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// Implementation of LimitTracker methods
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // Constructor
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // Updates the value and sends alerts based on usage
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You have used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You have used up over 75% of your quota");
        }
    }
}

// Main function to demonstrate usage
fn main() {
    let messenger = ConsoleMessenger;
    let mut tracker = LimitTracker::new(&messenger, 100);

    tracker.set_value(50);  // No warning
    tracker.set_value(80);  // Triggers 75% warning
    tracker.set_value(95);  // Triggers 90% warning
    tracker.set_value(110); // Triggers over-quota error
}

//The Messenger trait has one method called send that takes an immutable reference to self
//and the text of the message
//This trait is an interface the mock object needs to implement so that the mock can be used in the same way as a real object
//To test the behaviour of the set_value method on the LimitTracker, one can change the value of the parameters, but
//but, set_value does not return anything