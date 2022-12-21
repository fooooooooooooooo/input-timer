use std::collections::VecDeque;
use std::time::Instant;

use owo_colors::OwoColorize;
use press_btn_continue::wait;

fn main() {
  let mut durations: VecDeque<u128> = VecDeque::from(vec![0u128, 20]);

  wait("press any key to start\n").unwrap();

  let mut last_input_time = Instant::now();

  loop {
    wait("").unwrap();
    let now = Instant::now();
    let duration_nanos = (now - last_input_time).as_nanos();

    durations.pop_front();
    durations.push_back(duration_nanos);

    last_input_time = now;

    println!(
      "{} ms {}",
      format!("{: >9.4}", nanos_to_ms(duration_nanos)).bright_blue(),
      format!("(avg {: >9.4})", nanos_to_ms(avg_durations(&durations))).bright_black()
    );
  }
}

fn nanos_to_ms(nanos: u128) -> f64 { nanos as f64 / 1_000_000f64 }

fn avg_durations(vec: &VecDeque<u128>) -> u128 {
  let sum: u128 = vec.iter().sum();

  sum / vec.len() as u128
}
