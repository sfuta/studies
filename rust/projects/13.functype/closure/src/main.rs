use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("Calculationing slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", 
            simulated_expensive_calc(intensity)
        );

        println!(
            "Next, do {} situps!", 
            simulated_expensive_calc(intensity)
        );

    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", 
                simulated_expensive_calc(intensity)
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
fn main() {
    _use_generate_workout();
}