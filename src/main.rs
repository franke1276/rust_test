use std::time::{Duration, Instant};

fn test_for(to: i32) -> i64 {
    let mut sum: i64 = 0;
    let mut i: i32 = 0;

    for _y in 0..100 {
        i = 0;
        while i <= to {
            if i % 2 == 0 {
                sum += i as i64;
            }
            i += 1
        }
    }
    sum
}

fn test_while(to: i32) -> i64 {
    let mut sum: i64 = 0;

    for _y in 0..100 {
        for x in 0..to {
            if x % 2 == 0 {
                sum = sum + x as i64;
            }
        }
    }
    sum
}

fn test(f: &Fn(i32) -> i64) -> u64 {
    let to = 2000001;

    let start = Instant::now();

    let sum = f(to);

    let stop = Instant::now();
    let dur = stop.duration_since(start);

    dur.as_secs() as u64 * 1000000000 as u64 + dur.subsec_nanos() as u64
}

fn main() {
    let duration1 = test(&test_while);
    println!("duration while: {}", duration1 as f64 / (1000000.0 * 100.0));

    let duration2 = test(&test_for);
    println!("duration for: {}", duration2 as f64 / (1000000.0 * 100.0));
}
