use std::{fmt::Display, vec};

fn main() {

    let users: Vec<Box<dyn User>> = vec![Box::new(Employee {}), Box::new(Customer {})];


    let w  = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    

    println!("{}", w);

    // NOTE: this is type alias
    type Kilometer = i32;
}

// NOTE: ?Sized is a trait bound that means "T may or may not be a dynamically sized type"
fn generic<T: ?Sized>(t: &T) {

}

trait User {}

struct Employee {}

struct Customer {}

impl User for Employee {}

impl User for Customer {}
 
struct Wrapper(Vec<String>);


impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);
struct Id(u32);

// fn bar() -> ! {
//     // NOTE: this is never type
// }