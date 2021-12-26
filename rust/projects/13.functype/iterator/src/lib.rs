#[derive(PartialEq, Debug)]
struct Shoes {
    size: u32, style: String
}

fn _shoes_in_my_size(shoes: Vec<Shoes>, shoe_size: u32) -> Vec<Shoes> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}


struct Counter {
    count: u32
}
impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod my_tests {

    use super::*;

    #[test]
    fn using_other_iter_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                        .map(|(a, b)| a * b)
                        .filter(|x| x % 3 == 0)
                        .sum();
        
        assert_eq!(18, sum);
    }

    #[test]
    fn call_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoes { size: 10, style: String::from("sheaker") },
            Shoes { size: 13, style: String::from("sandal") },
            Shoes { size: 10, style: String::from("boot") },
        ];
        let in_my_size = _shoes_in_my_size(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoes { size: 10, style: String::from("sheaker") },
            Shoes { size: 10, style: String::from("boot") },
        ]);
    }

    #[test]
    fn itrator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}