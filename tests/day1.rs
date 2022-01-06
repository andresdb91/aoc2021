use std::fs;

use aoc2021::day1::day1p1;
use aoc2021::day1::day1p2;

#[test]
fn test_depth_increase() {
    let test_data = fs::read_to_string("day1-test.txt").unwrap();
    assert_eq!(day1p1::get_depth_increase(test_data.lines()), 7)
}

#[test]
fn test_sliding_window_increase() {
    let test_data = fs::read_to_string("day1-test.txt").unwrap();
    assert_eq!(day1p2::get_sliding_window_increase(test_data.lines()), 5)
}
