use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::Rc,
};
use crate::List::{Cons, Nil};

fn main() {
    // NOTE:
    // RefCell<T> - a mutable memory location with dynamically checked borrow rules
    // borrow rules is:
    // - At any given time, you can have either one mutable reference or any number of immutable references
    // - References must always be valid

    // let x = RefCell::new(122);

    // let y = &x;

    // println!("before mut the x\nx = {:?}", x);

    // *y.borrow_mut() += 1;

    // println!("After:");
    // println!("x = {:?}", x);
    // println!("y = {:?}", y);

    // let ref_cell_vec = RefCell::new(vec![1, 2, 3, 4]);

    // println!("Before {:#?}", ref_cell_vec.borrow());

    // for item in ref_cell_vec.borrow_mut().iter_mut() {
    //     *item += 1;
    // }

    // println!("After {:#?}", ref_cell_vec.borrow());

    let mut value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;
    value.borrow_mut().replace(10);

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used over 90% if your quota!")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        fn send(&self, msg: &str) {
            // let mut one_borrow_mut = self.sent_messages.borrow_mut();
            // let mut two_borrow_mut = self.sent_messages.borrow_mut();

            // one_borrow_mut.push(String::from(msg));
            // two_borrow_mut.push(String::from(msg));

            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_send_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 1);

        limit_tracker.set_value(100);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
