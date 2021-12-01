use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input:&str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn part1(depths: &[i32]) -> i32 {
    let windows = depths.windows(2).collect::<Vec<_>>();
    return windows.iter().fold(0, |sum, window| {
        if (window[1] > window[0]) {
            return sum + 1;
        } else {
            return sum;
        }
    });
}

#[aoc(day1, part2)]
pub fn part2(depths: &[i32]) -> i32 {
    let windows = depths.windows(3).collect::<Vec<_>>();
    let window_pairs = windows.windows(2).collect::<Vec<_>>();
    return window_pairs.iter().fold(0, |sum, pair| {
        let sumOne: i32 = pair[0].iter().sum();
        let sumTwo: i32 = pair[1].iter().sum();
        if (sumTwo > sumOne) {
            return sum + 1;
        } else {
            return sum;
        }
    });
}
