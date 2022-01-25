fn _fn_pointer() {

    fn add_one(x:i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    let answer_sub = do_twice(|x| x + 2, 5);
    println!("The answer is {}", answer);
    println!("The sub answer is {}", answer_sub);
}

fn main() {
    _fn_pointer();
}
