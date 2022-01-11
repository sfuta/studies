struct Point {
    x: i32, y: i32,
}

fn _struct() {
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

fn _ref() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum of points square {}", sum_of_squares);
}

fn _struct_taple() {
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);
}

fn main() {
    _struct();
    _ref();
    _struct_taple();
}