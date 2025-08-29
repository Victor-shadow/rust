pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTrackerObject<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTrackerObject<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTrackerObject<'a, T> {
        LimitTrackerObject {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_values(&mut self, value: usize) {
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

fn main() {
    // Example usage (optional)
    struct ConsoleMessenger;
    impl Messenger for ConsoleMessenger {
        fn send(&self, msg: &str) {
            println!("Message: {}", msg);
        }
    }

    let messenger = ConsoleMessenger;
    let mut tracker = LimitTrackerObject::new(&messenger, 100);
    tracker.set_values(80); // Should trigger 75% warning
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTrackerObject::new(&mock_messenger, 100);

        tracker.set_values(80); // 80% of 100 triggers 75% warning

        let messages = mock_messenger.sent_messages.borrow();
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0],
            "Warning: You have used up over 75% of your quota"
        );
    }
}

//the sent_message field is now of type RefCell<Vec<String>> instead of Vec<String>
//In the new function, anew RefCell<Vec<String>> is created instance around the empty vector
//For the implementation of the send method, the first parameter is still an immutable borrow of self, which
//matches the trait definition
//We call borrow_mut  on the RefCell<Vec<String>> in self.sent_messages to get the mutable reference to the value inside
//RefCell<Vec<String>> which is the vector
//Then, one can call the push method on the mutable reference to the vector to keep track of the messages sent during the test
//the last change to be made is in the assertion; to see how many items are in the inner vector
//we call borrow on RefCell<Vec<String>> to get an immutable reference to the vector