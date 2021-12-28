extern crate add_one;
// NG: rand imported add_one only
// extern crate rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
