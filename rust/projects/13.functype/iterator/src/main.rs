
fn _sample() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for v in v1_iter {
        println!("Got: {}", v);
    }
}
fn main() {
    _sample();
}
