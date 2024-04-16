use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct CustomData(i32);

fn main() {
    // NOTE:
    // Rc - reference counting smart pointer, it's count the number of references to a value
    // let a = Rc::new(List::Cons(21, Rc::new(List::Nil)));
    // let b = Rc::new(List::Cons(21, Rc::new(List::Cons(1, Rc::clone(&a)))));
    // let c = Rc::new(List::Cons(3, Rc::clone(&a)));

    let a = Rc::new(CustomData(21));
    let b = Rc::clone(&a);

    println!("reference count of a: {}", Rc::strong_count(&a));

    let c = Rc::clone(&a);
    println!("reference count of a: {}", Rc::strong_count(&a));

    {
        let d = Rc::clone(&a);
        println!("reference count of a: {}", Rc::strong_count(&a));
    }

    println!("reference count of a: {}", Rc::strong_count(&a));
}
