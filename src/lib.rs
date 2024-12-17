use std::collections::HashMap;

/// Processes each stone in the given collection. This is repeeated `blinks` amount of times,
/// and the result is returned as a new collection of stones.
///
/// ```
/// use stones::apply_blink;
/// assert_eq!(apply_blinks(&[125, 17], 1), vec![253000, 1, 7]);
/// assert_eq!(apply_blinks(&[125, 17], 2), vec![253, 0, 2024, 14168]);
/// assert_eq!(apply_blinks(&[125, 17], 3), vec![512072, 1, 20, 24, 28676032]);
/// ```
pub fn apply_blinks(intial_stones: &[usize], times: usize) -> Vec<usize> {
    if times == 0 {
        return intial_stones.to_vec();
    }

    let mut stones = apply_blink(intial_stones);

    for _ in 1..times {
        stones = apply_blink(&stones);
    }

    stones
}

/// Processes each stone in the collection, but returns only the total amount of stones in the results.
/// If you need to look at the actual values, use `blink` instead;
///
/// ```
/// use stones::count_stones_after_blinks;
/// assert_eq!(count_stones_after_blinks(&[125, 17], 6), 22);
/// assert_eq!(count_stones_after_blinks(&[125, 17], 25), 55312);
/// ```
pub fn count_stones_after_blinks(initial_stones: &[usize], blinks: usize) -> usize {
    initial_stones
        .iter()
        .map(|stone| count_stone_descendants(*stone, blinks, &mut HashMap::new()))
        .sum()
}

/// Processes each stone and appends its output - being either one or two stones - to the results
fn apply_blink(stones: &[usize]) -> Vec<usize> {
    let mut results = Vec::new();

    for n in stones {
        let (a, b) = process_stone(*n);

        results.push(a);

        if let Some(b) = b {
            results.push(b);
        }
    }

    results
}

/// Wraps `process_stone` but does not return the output - instead it only counts the "descendants" (including itself)
/// In other words, given X amount of steps, this function is able to calculate how many times the stone will split.
/// Additionally, the results can be cached, as they are idempotent given the input and amount of steps.
fn count_stone_descendants(
    input: usize,
    steps: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if steps == 0 {
        return 1;
    };

    if let Some(cached) = cache.get(&(input, steps)) {
        return *cached;
    }

    let result = match process_stone(input) {
        (left, None) => count_stone_descendants(left, steps - 1, cache),
        (left, Some(right)) => {
            count_stone_descendants(left, steps - 1, cache)
                + count_stone_descendants(right, steps - 1, cache)
        }
    };

    cache.insert((input, steps), result);

    result
}

/// Processes the input number according to the rules of AOC 2024, day 11.
/// It may return either one or two numbers. See unit tests for examples.
///
/// Rules:
/// 1. An input of 0 should result in 1
/// 2. An input of any number with an even amount of digits should be split in the middle, with any leading zeroes in the second half discarded:
/// 3. Any other numbers get multiplied by 2024:
fn process_stone(input: usize) -> (usize, Option<usize>) {
    if input == 0 {
        return (1, None);
    }

    let digits = count_digits(input);

    if digits % 2 == 0 {
        let (left, right) = split_number(input, digits);

        return (left, Some(right));
    }

    (input * 2024, None)
}

/// Returns the number of digits in a number
/// For example, `123` produces `3`, `10` produces `2` etc.
fn count_digits(n: usize) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() + 1
    }
}

/// Splits a number by digits, meaning 1234 is split into 12 and 34
fn split_number(n: usize, digits: u32) -> (usize, usize) {
    let pow = 10_usize.pow(digits / 2);
    let left = n / pow;
    let right = n - left * pow;

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_digits_should_handle_example_cases() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(12345), 5);
    }

    #[test]
    fn split_number_should_handle_example_cases() {
        assert_eq!(split_number(12, 2), (1, 2));
        assert_eq!(split_number(1234, 4), (12, 34));
    }

    #[test]
    fn split_number_should_handle_uneven_amount_of_digits() {
        assert_eq!(split_number(0, 1), (0, 0));
        assert_eq!(split_number(123, 3), (12, 3));
    }

    #[test]
    fn process_stone_should_handle_example_cases_according_to_the_rules() {
        assert_eq!(process_stone(0), (1, None));
        assert_eq!(process_stone(1000), (10, Some(0)));
        assert_eq!(process_stone(3), (6072, None));
    }

    #[test]
    fn blink_once_should_handle_example_case() {
        assert_eq!(
            apply_blink(&[0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }

    #[test]
    fn blink_should_handle_example_cases() {
        let initial = [125, 17];

        let rounds = [
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        ];

        for (index, output) in rounds.into_iter().enumerate() {
            assert_eq!(apply_blinks(&initial, index + 1), output);
        }
    }

    #[test]
    fn blink_should_provide_the_right_amount_of_stones() {
        let initial = [125, 17];
        assert_eq!(apply_blinks(&initial, 6).len(), 22);
        assert_eq!(apply_blinks(&initial, 25).len(), 55312);
    }

    #[test]
    fn count_stones_should_return_the_correct_count() {
        let initial = [125, 17];
        assert_eq!(count_stones_after_blinks(&initial, 6), 22);
        assert_eq!(count_stones_after_blinks(&initial, 25), 55312);
    }

    #[test]
    fn count_stone_descendants_should_return_the_right_count() {
        // 1700 -> 17,0
        assert_eq!(count_stone_descendants(1700, 1, &mut HashMap::new()), 2);
        // 1700 -> 17,0 -> 1,7,1
        assert_eq!(count_stone_descendants(1700, 2, &mut HashMap::new()), 3);
        // 1700 -> 17,0 -> 1,7,1 -> 2024,14168,2024
        assert_eq!(count_stone_descendants(1700, 3, &mut HashMap::new()), 3);
        // 1700 -> 17,0 -> 1,7,1 -> 2024,14168,2024 -> 20,24,28676032,20,24
        assert_eq!(count_stone_descendants(1700, 4, &mut HashMap::new()), 5);
    }
}
