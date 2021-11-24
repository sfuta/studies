struct User {
    name: String, email: String, sign_in_count: u64, is_actived: bool,
}
/**
 * Struct write rule
 */
fn _simple() {

    let mut user1 = User {
        email: String::from("abc@exam.com"), name: String::from("abc"), 
        sign_in_count: 3, is_actived: true,
    };

    // Show waring when field is not used as follow.
    user1.email = String::from("abc2@exam.com");
    user1.name = String::from("abc2");
    user1.sign_in_count = 4;

    let user2 = User {
        is_actived: false, ..user1
    };
    // Above is follow same
    // let user2 = User {name: user1.name, email: user1. email, sign_in_count: user1.sign_in_count, is_actived: false};

    if user1.is_actived {
        println!("user1 is active");
    }
    if user2.is_actived {
        println!("user1 is active");
    }
    struct Color(u32, u32, i32);
    struct Point(u32, u32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}, {}", black.0, origin.0);
}
fn _ret_struct1(email: String, name: String) -> User {
    return User {email: email, name: name, sign_in_count: 1, is_actived: true};
}
fn _ret_struct2(email: String, name: String) -> User {
    // Field name of 'email' and 'name' is omitted. (When Field name = value)
    return User {email, name, sign_in_count: 1, is_actived: true};
}

/**
 * Example 01
 */
#[derive(Debug)] // for use trait.
struct Rectangle { width: u32, height: u32, }
fn _example01() {
    let rect: Rectangle = Rectangle { width:20, height:10};
    println!("The area is '{}(m2)'", __area(&rect));

    // Use trait
    println!("{:?}", rect);
    println!("{:#?}", rect);
}
fn __area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/**
 * Use method
 */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn is_holded(&self, target: &Rectangle) -> bool {
        self.width > target.width
        && self.height > target.height
    }
    // depended method
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}
// impl block may be separated
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn _use_method() {
    let rect = Rectangle {width: 25, height: 16};
    println!("area is {}", rect.area());

    let rect_base = Rectangle {width: 40, height: 40};
    let rect_c1 = Rectangle {width: 39, height: 39};
    let rect_c2 = Rectangle {width: 40, height: 39};
    let rect_c3 = Rectangle {width: 39, height: 40};
    println!("Is holded rect_c1?: '{}'", rect_base.is_holded(&rect_c1));
    println!("Is holded rect_c2?: '{}'", rect_base.is_holded(&rect_c2));
    println!("Is holded rect_c3?: '{}'", rect_base.is_holded(&rect_c3));

    let square = Rectangle::square(20);
    println!("'square' properties is {:?}", square);

    println!("Is 'rect' square?:{}", rect.is_square());
    println!("Is 'square' square?:{}", square.is_square());
}


fn main() {
    _simple();
    _example01();
    _use_method();
}
