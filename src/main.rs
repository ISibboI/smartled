use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rppal::pwm::{Channel, Polarity, Pwm};

fn dim_triangle() {
    let pwm = Pwm::with_frequency(Channel::Pwm0, 1e3, 0.0, Polarity::Normal, true).unwrap();

    let mut intensity = 0.0;
    let mut increase = true;
    let benchmark_seconds = 10;
    let mut sleep_sum = Duration::ZERO;
    let start_time = Instant::now();

    for tick in 1..=benchmark_seconds * 1000 {
        let target_time = start_time + Duration::from_millis(tick);
        let current_time = Instant::now();
        if target_time > current_time {
            let sleep_time = target_time - current_time;
            sleep_sum += sleep_time;
            sleep(target_time - current_time);
        }

        if increase {
            intensity += 1e-3;
            if intensity > 1.0 {
                intensity = 1.0;
                increase = false;
            }
        } else {
            intensity -= 1e-3;
            if intensity < 0.0 {
                intensity = 0.0;
                increase = true;
            }
        }

        pwm.set_duty_cycle(intensity).unwrap();
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let duration = duration.as_secs_f64();
    println!("Duration factor: {}", duration / benchmark_seconds as f64);
    println!("Total sleep time: {}", sleep_sum.as_secs_f64());
}

fn main() {
    dim_triangle();
}
