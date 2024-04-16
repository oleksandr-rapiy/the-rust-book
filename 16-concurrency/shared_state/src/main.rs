use std::{sync::{Arc, Mutex}, thread::{self}};

#[derive(Debug)]
struct Data(i64);


#[derive(Clone, Debug)]
struct Counter(i64);

fn main() {
    // let data = Data(5);

    // let mutex = Mutex::new(data);

    // println!("Data = {:?}", mutex);

    // {
    //     let mut locked_data = mutex.lock().unwrap();
    //     locked_data.0 += 1;

    //     // NOTE: we don't need to unlock the mutex explicitly, mutex will be unlocked when it goes out of scope
    // }

    // println!("Data = {:?}", mutex);


    let counter = Arc::new(Mutex::new(Counter(0)));
    let mut threads = vec![];
    for _ in 1..10 {

        let mutex = Arc::clone(&counter);
        threads.push(
            
            thread::spawn(move || {
                let mut num = mutex.lock().unwrap();
                num.0 += 1;
                println!("Counter = {:?}", num.0);
            })
        );
    }

    for handle in threads {
        handle.join().unwrap();
    }

    println!("Counter = {:?}", Arc::clone(&counter));
}