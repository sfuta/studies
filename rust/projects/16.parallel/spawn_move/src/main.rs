use std::thread;

fn main() {
    let v = vec![1..3];

    // let handle = thread::spawn(||{
    let handle = thread::spawn(move ||{
        println!("Here's vector: {:?}", v);
    });

    // NG, v is moved
    // drop(v);

    handle.join().unwrap();
}
