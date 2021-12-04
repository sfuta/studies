
fn _vec() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(6);
    println!("v1[0]={:?}", &v1[0]);

    let mut v2 = vec![1, 2, 3];
    // let v3 = Vec::new(); NG
    let first = &v2[0];
    println!("first={:?}", first);
    v2.push(6);
    println!("v2[0]={:?}, v2[2]={:?}", &v2[0], v2.get(2));

    for i in &v2 {
        println!("{}", i);
    }

    let mut _v = vec![20, 26, 19];
    for i in &mut _v {
        *i += 50;
    }

    #[derive(Debug)]
    enum V { Int(i32), Float(f64), Text(String) }
    let rows = vec![V::Int(3), V::Text(String::from("abc")), V::Float(5.5), V::Int(4)];
    for row in &rows {
        print!("{:?}\t", row);
    }
    println!("");
}

fn _str() {
    let mut s = "aaaa".to_string(); // = String::from("aaaa")
    s.push_str(" bbbb");
    let t = " cccc";
    s.push_str(t);  // tの所有権は無くならない
    println!("t={}", t);
    s.push('.');
    println!("s={}", s);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world.");
    // let s3 = s1 + &s2; // s1 is move
    // println!("{}", s1);

    let s1 = String::from("2021");
    let s2 = String::from("12");
    let s3 = String::from("31");
    let _date = format!("{}-{}-{}", s1, s2, s3);
    println!("date={}", _date);

    let l = String::from("アイウエオ.");
    // let c = l[0]; NG: Rust is not support for index use access.
    let m = &l[0..6];
    println!("m={}", m);

    for c in l.chars() {
        print!("{} ", c);
    }
    print!("\n");

    for b in l.bytes() {
        print!("{} ", b);
    }
    print!("\n");

}

fn main() {
    _vec();
    _str();
}
