use std::time::Instant;
use stones::{apply_blinks, count_stones_after_blinks};

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("Expected 'apply' or 'count'");
    if command != "apply" && command != "count" {
        panic!("Command must be either 'apply' or 'count'")
    }
    let blinks = args.next().expect("Missing blink count");
    let blinks: usize = blinks
        .parse()
        .expect("Blinks should be a single non-negative integer");

    let mut input = args
        .map(|a| -> usize {
            a.parse()
                .expect("Stones should be space separated non-negative integers")
        })
        .collect::<Vec<_>>();

    if input.is_empty() {
        input = vec![125, 17]
    }

    println!("Blinking {blinks} times and {command}ing results for {input:#?}");

    let now = Instant::now();

    let count = if command == "apply" {
        apply_blinks(&input, blinks).len()
    } else {
        count_stones_after_blinks(&input, blinks)
    };

    println!("Count: {count:#?}");

    let duration = now.elapsed();

    println!(
        "Finished in {}.{} seconds",
        duration.as_secs(),
        duration.subsec_millis()
    )
}
