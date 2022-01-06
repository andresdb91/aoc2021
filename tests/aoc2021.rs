use std::fs;

use aoc2021::day1::*;

#[test]
fn test_depth_increase() {
    let test_data = fs::read_to_string("day1-test.txt").unwrap();
    assert_eq!(get_depth_increase(test_data.lines()), 7)
}

#[test]
fn test_sliding_window_increase() {
    let test_data = fs::read_to_string("day1-test.txt").unwrap();
    assert_eq!(get_sliding_window_increase(test_data.lines()), 5)
}
