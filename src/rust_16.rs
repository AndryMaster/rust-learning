use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;


pub fn f1_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();




    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
        println!("Thread id: {:?}", thread::current().id());
    });

    handle.join().unwrap();
}

pub fn f2_thread_channel() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = "hi".to_string();
    //     tx.send(val).unwrap();
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    let tx1 = tx;
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(501));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }   println!("End!");
}

pub fn f3_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for i in 0..5 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("th: {:?} \tnum: {}", thread::current().id(), *num);
                // drop(num);
                // thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
