use std::{sync::mpsc, thread};

fn main() {

    // // NOTE: mpsc stands for multiple producer, single consumer
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let msg = String::from("Message");

    //     // NOTE: here we are sending the message to the receiver
    //     tx.send(msg).unwrap();

    //     // NOTE: we can't use msg here because it has been moved to the receiver
    //     // println!("Sent: {}", msg);
    // });

    // let received = rx.recv().unwrap();

    // println!("Received: {}", received);


    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vec = vec![1, 2, 3, 4, 5];
        for i in vec {
            tx.send(i).unwrap();
            // thread::sleep(std::time::Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let vec = vec![6, 7, 8, 9, 10];
        for i in vec {
            tx2.send(i).unwrap();
            // thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
