use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // NG, val is borrowed already
        // println!("val is {}", val);
    });

    let revieved = rx.recv().unwrap();
    println!("Got: {}", revieved);

}
