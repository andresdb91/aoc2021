use std::fs;

use aoc2021::day1::get_depth_increase;
use aoc2021::day1::get_sliding_window_increase;
use aoc2021::day2::get_planned_course;
use aoc2021::day2::get_fixed_course;

fn main() {
    println!("Day 1");
    let data = fs::read_to_string("day1.txt").unwrap();
    println!("Part 1 - Depth increase rate: {}", get_depth_increase(data.lines()));
    println!("Part 2 - Sliding window increase rate: {}", get_sliding_window_increase(data.lines()));
    
    println!("Day 2");
    let data = fs::read_to_string("day2.txt").unwrap();
    let data = data.lines().collect();
    println!("Part 1 - Planned Course: {}", get_planned_course(&data));
    println!("Part 2 - Fixed Course: {}", get_fixed_course(&data));
}
