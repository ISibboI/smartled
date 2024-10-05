use std::{thread::sleep, time::Duration};

use rppal::pwm::{Channel, Polarity, Pwm};

fn main() {
    println!("Starting PWM");

    let mut pwm = Pwm::with_frequency(Channel::Pwm0, 1.0, 0.5, Polarity::Normal, true).unwrap();
    pwm.set_reset_on_drop(false);

    println!("PWM running, waiting forever");

    sleep(Duration::from_secs(u64::MAX));
}
