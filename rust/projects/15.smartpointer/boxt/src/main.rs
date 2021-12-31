// enum List {
//     Cons(i32, List), Nil,
// }
enum List {
    Cons(i32, Box<List>), Nil,
}
use List::{Cons, Nil};

fn _box() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    #[allow(unused_variables)]
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#[test]
fn _deref() {
    let x = 5; 
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // deref
}

use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
#[test]
fn _my_deref() {
    let x = 5; 
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // deref
}

fn main() {
    _box();
}
