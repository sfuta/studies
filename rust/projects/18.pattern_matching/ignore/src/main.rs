fn _all() {
    fn foo(_: i32, y: i32) {
        println!("Only uses the second parameter: {}", y);
    }

    foo(2000, 20);
}

fn _nest_part() {
    let mut set_val = Some(5);
    let new_set_val = Some(10);

    match(set_val, new_set_val) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            set_val = new_set_val;
        }
    }

    println!("setting is {:?}", set_val);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn _variable() {
    let _ignore = 5;

    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn main() {
    _all();
    _nest_part();
    _variable();
}
