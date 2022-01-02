// enum List {
//     Cons(i32, Box<List>), Nil
// }
// use List::{Cons, Nil};
// fn _ng_rc() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     // NG: a moved
//     let c = Cons(3, Box::new(a));
// }

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>), Nil
}
use List::{Cons, Nil};
fn _rc() {
    #[allow(unused_variables)]
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    #[allow(unused_variables)]
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        #[allow(unused_variables)]
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

extern crate others;

fn main() {
    _rc();
    others::rc_refcell::run();
    others::ref_cycle::run();
}
