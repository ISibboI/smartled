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
    let start_time = Instant::now();

    for _ in 0..benchmark_seconds * 1000 {
        sleep(Duration::from_millis(1));

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
}

fn main() {
    dim_triangle();
}
