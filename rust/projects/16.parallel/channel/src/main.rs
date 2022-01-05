use std::sync::mpsc;
use std::time::Duration;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // NG, val is borrowed already
        // println!("vals is {}", vals);
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }
    // let recieved = rx.recv().unwrap();
    // println!("Got: {}", recieved);

}
