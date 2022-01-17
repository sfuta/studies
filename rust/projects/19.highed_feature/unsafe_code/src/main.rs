fn _simple() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe fn dangerous() { println!("Call unsafe fn") }
    unsafe {
        dangerous();
    }
}

use std::slice;
fn _unsafe_fn() {
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    println!("a is {:?}", a);
    println!("b is {:?}", b);
}

extern "C" {
    fn abs(input: i32) -> i32;
}
fn _call_extern_fn() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn main() {
    _simple();
    _unsafe_fn();
    _call_extern_fn();
}
