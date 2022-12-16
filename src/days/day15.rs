use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::time::SystemTime;
use regex::{Captures, Match, Regex};
use crate::utility::utils::parse_file;

struct SensorReading {
    sensor: (i32, i32),
    beacon: (i32, i32)
}

impl Debug for SensorReading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) senses ({}, {})", self.sensor.0, self.sensor.1, self.beacon.0, self.beacon.1)
    }
}

fn read_input() -> Vec<SensorReading> {
    let sensor_rgx = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    parse_file("inputs/day15.txt", |line| {
        let c = sensor_rgx.captures(line).unwrap();
        
        SensorReading {
            sensor: (c.get(1).unwrap().as_str().parse::<i32>().unwrap(), c.get(2).unwrap().as_str().parse::<i32>().unwrap()),
            beacon: (c.get(3).unwrap().as_str().parse::<i32>().unwrap(), c.get(4).unwrap().as_str().parse::<i32>().unwrap()),
        }
    })
}

fn get_taxi_dist(reading: &SensorReading) -> i32 {
    (reading.sensor.0 - reading.beacon.0).abs() + (reading.sensor.1 - reading.beacon.1).abs()
}

fn get_spots(reading: &SensorReading, y: i32, ranges: &mut Vec<(i32, i32)>) {
    let base_range = get_range(&reading, y);
    if reading.beacon.1 == y {
        // combine_ranges((base_range.0, reading.beacon.0), ranges);
        // combine_ranges((reading.beacon.0 + 1, base_range.1), ranges);
        if reading.beacon.0 == base_range.0 {
            combine_ranges((base_range.0 + 1, base_range.1), ranges);
        } else {
            combine_ranges((base_range.0 , base_range.1 - 1), ranges);
        }
    } else {
        combine_ranges(base_range, ranges);
    }
}

fn get_range(reading: &SensorReading, y: i32) -> (i32, i32) {
    let y_dist = (reading.sensor.1 - y).abs();
    let taxi_dist = get_taxi_dist(&reading);
    let r = max(0, taxi_dist - y_dist);

    (reading.sensor.0-r, reading.sensor.0+r) // +1 cause end exclusive
}

fn merge_range(range: (i32, i32), other: (i32, i32)) -> (i32, i32) {
    (min(range.0, other.0), max(range.1, other.1))
}

fn combine_ranges(mut r: (i32, i32), ranges: &mut Vec<(i32, i32)>) {
    //println!("{:?} -> {:?}", r, ranges);
    if r.0 == r.1 {
        return;
    }
    
    if ranges.is_empty() {
        ranges.push(r);
        return;
    }
    
    let mut merge_start: i32 = -1;
    let mut merge_end: i32 = -1;
    for i in 0..ranges.len() {
        let current = ranges[i];
        if r.1 - current.0 < -1 {
            if merge_start < 0 {
                ranges.insert(i, r);
            }
            break;
        } else if r.0 - current.1 > 1 {
            if i == ranges.len() - 1 {
                ranges.push(r);
            }
        } else {
            if merge_start < 0 {
                merge_start = i as i32;
            }
            merge_end = i as i32;
            
            r = merge_range(r, current);
            //println!("Merged range {:?}", r);
        }
    }
    
    //println!("{}, {}", merge_start, merge_end);
    //     _ _
    // 0 1 2 3 4 5
    if merge_start >= 0 {
        for i in merge_start..merge_end+1 {
            ranges.remove(merge_start as usize);
        }
        ranges.insert(merge_start as usize, r);
    }
}

fn test_cr() {
    let mergee_1 = (1,1);
    let mut ranges_1 = vec![(1, 1)];
    let expected_1 = vec![(1, 1)];
    combine_ranges(mergee_1, &mut ranges_1);
    assert_eq!(ranges_1, expected_1);

    let mergee_2 = (2,3);
    let mut ranges_2 = vec![(5, 7)];
    let expected_2 = vec![(2, 3), (5, 7)];
    combine_ranges(mergee_2, &mut ranges_2);
    assert_eq!(ranges_2, expected_2);

    let mergee_3 = (2,5);
    let mut ranges_3 = vec![(5, 7)];
    let expected_3 = vec![(2, 7)];
    combine_ranges(mergee_3, &mut ranges_3);
    assert_eq!(ranges_3, expected_3);

    let mergee_3 = (2,5);
    let mut ranges_3 = vec![(5, 7)];
    let expected_3 = vec![(2, 7)];
    combine_ranges(mergee_3, &mut ranges_3);
    assert_eq!(ranges_3, expected_3);

    let mergee_4 = (3,4);
    let mut ranges_4 = vec![(2, 3), (4, 7)];
    let expected_4 = vec![(2, 7)];
    combine_ranges(mergee_4, &mut ranges_4);
    assert_eq!(ranges_4, expected_4);

    let mergee_5 = (9,10);
    let mut ranges_5 = vec![(2, 3), (4, 7)];
    let expected_5 = vec![(2, 3), (4, 7), (9, 10)];
    combine_ranges(mergee_5, &mut ranges_5);
    assert_eq!(ranges_5, expected_5);

    let mergee_6 = (2,2);
    let mut ranges_6 = vec![(2, 3), (4, 7)];
    let expected_6 = vec![(2, 3), (4, 7)];
    combine_ranges(mergee_6, &mut ranges_6);
    assert_eq!(ranges_6, expected_6);
    
    // (3070241, 3485039) -> [(2913315, 3549817), (3846541, 4121117)]
    let mergee_7 = (3070241, 3485039);
    let mut ranges_7 = vec![(2913315, 3549817), (3846541, 4121117)];
    let expected_7 = vec![(2913315, 3549817), (3846541, 4121117)];
    combine_ranges(mergee_7, &mut ranges_7);
    assert_eq!(ranges_7, expected_7);
}

pub fn solve() {
    let elapsed = SystemTime::now();
    let mut input = read_input();

    //test_cr();
    
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 15");
    println!("Part 1: {}", part1); // 4560025
    println!("Part 2: {}", part2); // 12480406634249
    println!("Elapsed time: {}ms", elapsed.elapsed().unwrap().as_millis());
}

fn solve_part1(readings: &Vec<SensorReading>) -> i32 {
    let mut ranges = vec![];
    for r in readings {
        get_spots(&r, 2000000, &mut ranges);
    }
    ranges.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn solve_part2(readings: &Vec<SensorReading>) -> u64 {
    for y in 0..4_000_000+1 {
        let mut ranges = vec![get_range(&readings[0], y)];
        for i in 1..readings.len() {
            //println!("{:?} --> {:?}", get_range(&readings[i], y), ranges);
            combine_ranges(get_range(&readings[i], y), &mut ranges);
        }
        //println!("{:?}", ranges);
        if ranges.len() > 1 {
            return (ranges[0].1 + 1) as u64 * 4_000_000 + y as u64;
        }
    }

    panic!("No result for part 2");
}
