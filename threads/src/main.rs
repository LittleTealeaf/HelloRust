use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    println!("Saving the value as a handle");

    // Saving the value
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
        // .join() waits for the thread to finish,
    }

    println!("Waiting for the handle before doing main thread");
    // what if it's before
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // .join() waits for the thread to finish,
    }

    // moving variables into the clusre
    println!("Moving vars into the closure");

    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    // communicating between threads!
    // WAIT THIS IS ACTUALLY COOL
    println!("Here's an example for sending data between threads!");
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            println!("Sending {} to mpsc channel", val);
            tx.send(val).unwrap();
            // val is no longer owned by this scope, so you cannot use it here
        });

        let recieved = rx.recv().unwrap();
        println!("Got: {}", recieved);

        // .recv() will block the thread until it recieves something
        // .try_recv() will check if it's recieved something, if not it'll return an Err value
    }

    println!("Here's an example for sending multiple data values");
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    println!("Multiple producers!");
    {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
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
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        for received in rx {
            println!("Got {}", received);
        }
    }

    println!("Share State Concurrency");
    {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    println!("Sharing between threads using Arc<T>");
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }


        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
