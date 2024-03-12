fn main() {
    /*
        NOTE: OWNERSHIP !!

        ownership model -> way to manage memory.

        props of ownership:
            control over memory
            error free* (compile type checks, inability to control over memory)
            faster runtime
            smaller program size

        const of ownership:
            slower write time. Learning curve (fighting with the borrow checker)
    */

    /*
        NOTE: OWNERSHIP RULES:

        1. Each value in RUST has a variable that's called its owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.

    */

    {
        // s is not valid here, not declared yet
        let s = "string literal"; // s is valid from this point forward
                                  // can do stuff with s

        let s = String::from("Some"); // this s stored in heap.
    } // the scope is now ower, and s is no longer valid here

    let x = 5;
    let y = x; // here we copy the value

    let str = String::from("some literal");
    let str2 = str; // move (not shallow copy)
    let str3 = str2.clone(); // to be able to use str2

    println!("{}", str2);

    // NOTE: takes ownership

    let s = String::from("strong");
    takes_ownership(s); // moves s to str variable
                        // println!("{}", s); // we can't use s here, it's moved to str

    let x = 321;
    makes_copy(x); // we makes a copy 'cause it's int type
    println!("{}", x);

    // NOTE: references and mutable references
    let s = String::from("strong");
    let len = calculate_length(&s);
    println!("len - {} {}", s, len);

    let mut s = String::from("value");
    update_str(&mut s);
    println!("{}", s);

    // NOTE: you can only one mut reference to the particular data or scope
    let mut s = String::from("value");
    let s2 = &mut s;
    let s3 = &mut s; // can't borrow mutable ref more than once
    println!("{},", s3);

    // NOTE: you cannot have mutable reference if immutable reference already exist
    let mut s = String::from("value");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // NOT ALLOWED

    println!("{}, {}", s1, s2);

    // NOTE: The Rules of references
    // 1. At any given time, you can have either one mutable reference or
    // any number of immutable references.
    //
    // 2. References must always be valid.

    // NOTE: Slice type
    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let s2 = "hello world";

    let word = first_word(&s);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3];
    for item in slice.iter() {
        print!("{} ", item)
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn takes_ownership(str: String) {
    println!("{}", str)
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

// we pass the reference of the string to function
fn calculate_length(str: &String) -> usize {
    return str.len();
}

// we pass the mutable reference of the string to the function
fn update_str(str: &mut String) {
    return str.push_str(" string");
}

fn stack_and_heap() {
    /*
        NOTE:

        Stack:
        b() x        -> x - stored in the heap, cause it's dynamic, in stack stored ref to the value in heap
        a() x, y     -> x and y - stored in the stack (String literals stored in stack)

        when the b() ends -> the 'x' will be dropped from the stack
        when the a() ends -> the 'x' and 'y' will be dropped from the stack


    */
    fn a() {
        let x = "hello";
        let y = 34;
        b();
    }

    fn b() {
        let x = String::from("Some string value");
    }
}
