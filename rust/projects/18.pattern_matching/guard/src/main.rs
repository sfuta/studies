fn _equal() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn _multi() {
    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        7 | 8  => println!("mid yes"),
        _ => println!("no")
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    _equal();
    _multi();
}
