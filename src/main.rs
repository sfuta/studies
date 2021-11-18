fn _data_types() {
    // // Rust data type;
    // let dt: u16 = 1_000;
    // let dt: isize = 1_000;
    // let dt: f32 = 1.0;
    // let dt = 1.0;   // f64
    // let t = true;
    // let f: bool = false;
    // let c = 'c'; // char type
    // let c = '感'; // char type
    // let c = 'abc'; // NG, because abc is not char
}

fn _tup() {
    let tup0: (i32, f64, u8) = (500, 6.4, 1);
    // let tup0 = (500, 6.4, 1);
    let (x, y, z) = tup0;

    println!("The value of {}, {}, {}", x, y, z);
}

fn _array() {
    let arr = [1, 2, 3, 4];
    let arr0 = arr[0];
    println!("The value of arr0 is {}", arr0);
}

// statement(文) and expression(式)
fn _s_e(param: bool) {
    // NG, because "let y = 6" is statement
    // let x = (let y = 6);
    // OK, because "{ let x = 3; x + 1 };" is expression
    let y = { let x = 3; x + 1 };
    println!("The value of y is {}, param is {}", y, param)
}

fn _return0() -> i32 {
    5
}
// NG, because "5;" is a statement
// fn _return1() -> i32 {
//     5;
// }

/**
 * if
 **/
fn _if() {
    // simple
    let num = 3;
    if num < 4 {
        println!("simple!!")
    }
    // NG, because if condition is required bool type
    // if num {
    //     // error
    // }

    // Following OK, because if is expression
    let a = if true {
        5
    } else {
        6
    };
    println!("The value of a in _if() is {}", a);
}
fn main() {
    _if();
    println!("The return value of _return0() is {}", _return0());
    _s_e(false);
    _array();
    _tup();
    _data_types();

    let mut x0 = 5;
    println!("The value of x0 is {}", x0);
    x0 = 6;
    println!("The value of x0 is {}", x0);

    let x1 = 3;
    let x1 = x1 + 1;
    let x1 = x1 * 2;

    println!("The value of x1 is {}", x1);

    // OK
    let spaces = "      ";
    let spaces = spaces.len();
    // NG
    // let mut spaces = "      ";
    // spaces = spaces.len();
    println!("Space time is {}", spaces);

    _data_types();
}
