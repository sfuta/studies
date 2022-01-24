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

fn _never() {
    // TODO 実装方法が不明
    // never typeはcontinue, println!(), panic!等で使用
    // fn _return_empty(s: String) -> ! {
    // }

    // _return_empty(String::from("56"));
}

fn _dynamic_sized() {
    // &str等のスライス型が動的型付け

    // 動的な型付けとなる
    fn _dyn_sized<T>(_t: T) {
        // TODO
    }
    // -> ※下記のように扱われる
    fn _express_dyn_sized<T: Sized>(_t: T) {
        // TODO
    }

    fn _loose_dyn_sized<T: ?Sized>(_t: &T) {
        // TODO
    }
}

fn main() {
    _alias();
    _never();
    _dynamic_sized();
}
