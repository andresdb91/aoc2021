use std::fs;

use aoc2021::day1::*;
use aoc2021::day2::*;

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

#[test]
fn test_planned_course() {
    let test_data = fs::read_to_string("day2-test.txt").unwrap();
    let test_data = test_data.lines().collect();
    assert_eq!(get_planned_course(&test_data), 150)
}

#[test]
fn test_fixed_course() {
    let test_data = fs::read_to_string("day2-test.txt").unwrap();
    let test_data = test_data.lines().collect();
    assert_eq!(get_fixed_course(&test_data), 900)
}
