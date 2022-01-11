fn _all() {
    fn foo(_: i32, y: i32) {
        println!("Only uses the second parameter: {}", y);
    }

    foo(2000, 20);
}
fn main() {
    _all();
}
