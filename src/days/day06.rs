use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> String {
    let contents = match read_to_string("inputs/day06.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    
    contents.to_string()
}

pub fn solve() {
    let input = read_input();

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 06");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_marker(data: &String, distinct: usize) -> usize {
    let mut i = 0;
    let mut chars = HashSet::new();
    while chars.len() != distinct {
        let sliding_window = &data[i..i+distinct];
        chars.clear();
        for c in sliding_window.chars() {
            chars.insert(c);
        }
        i += 1
    }

    i - 1 + distinct
}

fn solve_part1(data: &String) -> usize {
    find_marker(data, 4)
}

fn solve_part2(data: &String) -> usize {
    find_marker(data, 14)
}
