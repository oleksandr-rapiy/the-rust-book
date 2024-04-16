use std::{cell::RefCell, rc::{Rc, Weak}};


#[derive(Debug)]
struct Node{
    value: i32, 
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

// NOTE: 
// Weak smart pointer is used to avoid reference cycles.

fn main() {
    let leaf = Rc::new(Node {
        value: 3, 
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


    let branch = Rc::new(Node {
        value: 5, 
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent  = {:#?}", leaf.parent.borrow().upgrade());


    let user = User::new(12, String::from("User name"));



}


struct Id { 
    id: i64
}

struct User {
    id: Id,
    name: String
}

impl Id {
    fn new(id: i64) -> Self {
        if id < 0 || id == 12  {
            panic!("Invalid id");
        }

        Id { id }
    }
}

impl User {    
    fn new(id: i64, name: String) -> Self {
        let id = Id::new(id);

        User {
            id,
            name
        }
    }
}