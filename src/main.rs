use std::{thread::sleep, time::Duration};

use rppal::pwm::{Polarity, Pwm};

fn main() {
    println!("Starting PWM");

    let _ =
        Pwm::with_frequency(rppal::pwm::Channel::Pwm0, 1.0, 0.5, Polarity::Normal, true).unwrap();

    println!("PWM running, waiting forever");

    sleep(Duration::from_secs(u64::MAX));
}
