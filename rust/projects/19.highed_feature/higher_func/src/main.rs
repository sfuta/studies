fn _fn_pointer() {

    fn add_one(x:i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    let answer_01 = do_twice(|x| x + 2, 5);
    println!("The answer is {}", answer);
    println!("The answer is {}", answer_01);
}

fn _return_closure() {

    // fn add_one_fn() -> Fn(i32) -> i32() NG
    fn add_one_fn() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| { x + 1 })
    }

    println!("The answer is {}", add_one_fn()(3));
}

fn main() {
    _fn_pointer();
    _return_closure();
}
