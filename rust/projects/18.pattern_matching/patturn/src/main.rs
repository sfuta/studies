fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at end: x = {:?}, y = {:?}", x, y);
    // -> Output is the follow
    // Matched, y = 5
    // at end: x = Some(5), y = 10

    let x = 1;

    match x {
        1 | 2 => println!("one or twe"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("something else"),
    }
}
