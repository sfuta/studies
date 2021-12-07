fn _find_max_v() {

    // pattern 1
    let nums = vec![40, 50, 33, 6];
    let mut largest = nums[0];
    for num in nums {
        if num > largest {
            largest = num; 
        }
    }
    println!("Max value is {} in 'nums'", largest);

    // pattern 2
    let nums = vec![40, 60, 50, 33, 6];
    fn find_max_from(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    println!("Max value is {} in 'nums'", find_max_from(&nums));
}
fn main() {
    _find_max_v();
}
