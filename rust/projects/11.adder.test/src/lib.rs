#[derive(Debug)]
pub struct Rectangle { length: u32, width: u32 }
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn another() {
    //     // This test may fail
    //     panic!("Make this fail");
    // }


}
