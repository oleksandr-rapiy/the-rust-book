use List::{Cons, Nil};

enum List {

    // NOTE: Box smart is a fix size pointer 
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let b = Box::new(10);

    println!("b = {}", b);


    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}