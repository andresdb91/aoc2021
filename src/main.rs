use std::fs;

use aoc2021::day1::day1p1::get_depth_increase;
use aoc2021::day1::day1p2::get_sliding_window_increase;

fn main() {
    println!("Day 1");
    let day1_data = fs::read_to_string("day1.txt").unwrap();
    println!("Part 1 - Depth increase rate: {}", get_depth_increase(day1_data.lines()));
    println!("Part 2 - Sliding window increase rate: {}", get_sliding_window_increase(day1_data.lines()));
}
