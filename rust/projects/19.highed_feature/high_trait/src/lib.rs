use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn meters_add() {
        let a: Meters = Meters(3);
        let b: Millimeters = Millimeters(3);

        assert_eq!(Millimeters(3003), b + a);
    }
}
