fn _alias() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        f();
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("return"))
    }

    f();
    takes_long_type(Box::new(|| println!("takes")));
    returns_long_type()();
}

fn main() {
    _alias();
}
