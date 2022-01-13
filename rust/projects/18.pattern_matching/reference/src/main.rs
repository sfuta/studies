fn _immutable() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => {},
    }

    println!("robot_name is: {:?}", robot_name);

}

fn _mutable() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => {
            *name = String::from("Another name");
        },
        None => {},
    }

    println!("robot_name is: {:?}", robot_name);

}

fn main() {
    _immutable();
    _mutable();
}
