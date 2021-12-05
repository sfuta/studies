
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

/**
 * String
 */
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

/**
 * Hash map
 */
fn _map() {
    use std::collections::HashMap;

    let mut scores: HashMap<String,u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 22);

    let teams = vec!["Blue", "Yellow"];
    let init_score_values = vec![8, 40];
    
    let init_scores: HashMap<_, _> = teams.iter().zip(init_score_values.iter()).collect();
    println!("map={:?}", init_scores);

    let f1 = String::from("First");
    let v1 = String::from("First Value");
    let mut map1: HashMap<String,String> = HashMap::new();
    map1.insert(f1, v1);
    // f1,v1 is moved
    // println!("{},{}", f1,v1);

    // HashMap for
    for (k, v) in &scores {
        println!("(team, score) = ({}, {})", k, v);
    }
    print!("\n");

    // HashMap::get()
    println!("Blue team score={:?}", scores.get("Blue"));
    // HashMap value update
    scores.insert(String::from("Blue"), 40);
    // HashMap value insert when value no exists
    scores.entry(String::from("Yellow")).or_insert(65);
    scores.entry(String::from("Blue")).or_insert(65);
    scores.entry(String::from("Red")).or_insert(65);
    for (k, v) in &scores {
        println!("(team, score) = ({}, {})", k, v);
    }

    let text = "aaa bbb ccc ddd aaa";
    let mut map: HashMap<&str, i8> = HashMap::new();
    for word in text.split_whitespace() {
        // HashMap::entry() return value is map value
        let count = map.entry(word).or_insert(0);
        // value is update
        *count += 1;
    }
    println!("map={:?}", map);
}

fn main() {
    _vec();
    _str();
    _map();
}
