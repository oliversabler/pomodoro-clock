use std::{thread, time};

// Time
const POMODORO: u64 = 25 * 60; // 25 minutes
const SHORT_BREAK: u64 = 5 * 60; // 5 minutes
const LONG_BREAK: u64 = 15 * 60; // 15 minutes

// Test Time
// const POMODORO: u64 = 10;
// const SHORT_BREAK: u64 = 2;
// const LONG_BREAK: u64 = 5;

const MAX_POMODORO_COUNT: u8 = 3;

#[derive(Debug)]
enum Kind {
    Pomodoro,
    ShortBreak,
    LongBreak
}

fn main() {
    let mut pomodoro_iter = 1;
    loop {
        run_timer(Kind::Pomodoro);
        // Todo: replace with let if or match?
        if pomodoro_iter >= MAX_POMODORO_COUNT {
            run_timer(Kind::LongBreak);
            pomodoro_iter = 1
        } else {
            run_timer(Kind::ShortBreak);
            pomodoro_iter += 1;
        }
    }
}

fn run_timer(kind: Kind) {
    println!("Running timer for {:?}", kind);

    let timer = time::Duration::from_secs(set_timer(kind));
    let start_time = time::Instant::now();

    let mut previous_second: u64 = timer.as_secs();

    loop {
        let elapsed_time = start_time.elapsed();
        if elapsed_time >= timer {
            println!("Done!");

            let sleep_time = time::Duration::from_secs(1);
            thread::sleep(sleep_time);

            break;
        }

        let remaining_seconds = (timer - elapsed_time).as_secs();
        
        if second_updated(remaining_seconds, previous_second) {
            previous_second = remaining_seconds;
            println!("Time remaining: {}s", remaining_seconds + 1);
        }
    }
}

fn set_timer(kind: Kind) -> u64 {
    match kind {
        Kind::Pomodoro => POMODORO,
        Kind::ShortBreak => SHORT_BREAK,
        Kind::LongBreak => LONG_BREAK
    }
}

fn second_updated(remaining_seconds: u64, previous_second: u64) -> bool {
    new_second(remaining_seconds, previous_second) || new_timer(remaining_seconds, previous_second)
}

fn new_second(remaining_seconds: u64, previous_second: u64) -> bool {
    remaining_seconds < previous_second
}

fn new_timer(remaining_seconds: u64, previous_second: u64) -> bool {
    remaining_seconds > previous_second && previous_second == 0
}
