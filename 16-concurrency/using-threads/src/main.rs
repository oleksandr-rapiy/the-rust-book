use std::{thread, time::Duration};

fn main() {

     thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }).join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();

    let vec = vec![1, 2, 3, 4, 5, 6];
    
    let handle = thread::spawn(move || {
        println!("Vec = {:#?}", vec);
        // for i in vec {
        //     println!("i: {}", i);
        // }
    });

    handle.join().unwrap();
}
