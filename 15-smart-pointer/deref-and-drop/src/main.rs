
use std::{fmt::Debug, ops::{Deref, DerefMut}};

#[derive(Debug)]
struct MyBox<T>(T) where T: std::fmt::Debug;

impl<T> MyBox<T> where T: Debug {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for MyBox<T> where T : Debug {
    // NOTE: associated type Target is used to define the type of value
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> where T : Debug {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// NOTE: drop trait is used to define the behavior when the value goes out of scope
// the same as IDisposable in C#
impl<T> Drop for MyBox<T> where T : Debug{
    fn drop(&mut self) {
        println!("Dropping MyBox with value: {:#?}", self.0);
    }
}

fn main() {
    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);

    // NOTE: dereference operator * used to access the value of y
    // which is a reference to x
    assert_eq!(5, *y);

    {
        let z = MyBox::new(x);

        assert_eq!(5, *z);
    }
    
    let m = MyBox::new(String::from("Rasty"));

    hello(&m);
    drop(m);

    // &MyBox<String> -> &String -> &str



    // NO
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}