use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join(); // Wait for the spawned thread


    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // 'move' forces the closure to take ownership of the values it uses
        println!("Here's a vector: {:?}", v);
    });

    handle.join();


    // Message passing through channels

    use std::sync::mpsc;    
    use std::time::Duration;

    fn send_and_sleep<T>(val: T, tx: &mpsc::Sender<T>, seconds: u8) {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    let (tx1, rx) = mpsc::channel(); // Multiple Producers, Single Consumer (mpsc) channel
    
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || { // Takes ownership of tx1
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            send_and_sleep(val, &tx1, 1)
            //tx1.send(val).unwrap();
            //thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || { // Takes ownership of tx2
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            send_and_sleep(val, &tx2, 1)
            //tx.send(val).unwrap();
            //thread::sleep(Duration::from_secs(1));
        }
    });    

    for received in rx {
        println!("Got: {}", received); // Messages will be undeterministically unordered
    }


    // Mutex allows access to shared data from one thread at a time

    use std::sync::{ Mutex, Arc };

    // The mutex will be moved but we need multiple references to it => Arc: thread-safe version of the  Rc smart pointer
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
