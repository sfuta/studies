
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

}

fn main() {
    _vec();
}
