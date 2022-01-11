struct Point {
    x: i32, y: i32,
}

fn main() {
    let p1 = Point { x: 0, y: 7 };
    let p2 = Point { x: 0, y: 7 };
    let p3 = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p1;
    println!("a = {}, b = {}", a, b);

    let Point { x, y } = p2;
    println!("x = {}, y = {}", x, y);

    match p3 {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}