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

struct Point {
    x: i32, y: i32, z: i32,
}

fn _remnant() {

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

}

fn _mid() {

    let numbers = (2, 4, 6, 8, 10, 12);

    match numbers {
        (f, .., e) => {
            println!("Some numbers: {}, {}", f, e);
        }
    }

}

fn main() {
    _all();
    _nest_part();
    _variable();
    _remnant();
    _mid();
}
