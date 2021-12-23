use std::thread;
use std::time::Duration;

struct Cacher<T: Fn(u32) -> u32> {
    calc : T, value: Option<u32>
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calc: T) -> Cacher<T> {
        Cacher { calc, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("Calculationing slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", 
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!", 
            expensive_result.value(intensity)
        );

    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", 
                expensive_result.value(intensity)
            );
        }
    }

}
fn _use_generate_workout() {

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value, simulated_random_number
    )

}

fn _with_external_variable() {
    let x = 4;
    let equal_to_x = |z:u32| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z:Vec<u32>| { z == x };
    // println!("can't use x here: {:?}, because x is moved", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
fn main() {
    _use_generate_workout();
    _with_external_variable();
}