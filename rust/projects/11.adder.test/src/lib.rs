#[derive(Debug)]
pub struct Rectangle { length: u32, width: u32 }
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    // format!("Hello {}!", _name)
    format!("Hello")
}

pub struct Guess {
    #[allow(dead_code)]
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        return Guess { value };
    }

    pub fn get_instance(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be grater than or equal to 1, got {}", value);
        }

        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        
        Guess { value }
    }
}

fn _prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod sub {

    #[test]
    fn sub_test() {
        let v = vec![5, 2];
        assert_eq!(vec![5, 2], v);
    }
}

#[cfg(test)]    // The execute is 'cargo test' command only
mod ttt {
// mod tests {
    use super::*;

    // it is executed command with cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        let v = 9;
        assert_eq!(9, v);
    }

    // std output diplaied command, cargo test -- --nocapture
    #[test]
    fn this_test_will_pass() {
        let value = _prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = _prints_and_return_10(8);
        assert_eq!(5, value);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn throw_panic_greater_than_100() {
        Guess::get_instance(200);
    }
    #[test]#[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol")
            , "Greeting did not contain name, value was '{}'", result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_eq!(5, add_two(2));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn expression() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // This test may fail
        panic!("Make this fail");
    }


}
