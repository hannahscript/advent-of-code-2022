use super::super::utility::utils::parse_file;

type Range = (i32, i32);

fn read_input() -> Vec<(Range, Range)> {
    parse_file("inputs/day04.txt", |line| {
        let elves: Vec<&str> = line.split(",").collect();
        (parse_range(elves[0]), parse_range(elves[1]))
    })
}

fn parse_range(text: &str) -> Range {
    let pair: Vec<&str> =  text.split("-").collect();
    return (pair[0].parse::<i32>().unwrap(), pair[1].parse::<i32>().unwrap());
}

fn is_subrange(range: Range, candidate: Range) -> bool {
    candidate.0 >= range.0 && candidate.1 <= range.1
}

fn overlaps(a: Range, b: Range) -> bool {
    !(a.1 < b.0 || b.1 < a.0)
}

pub fn solve() {
    let input = read_input();
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 04");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(assignments: &Vec<(Range, Range)>) -> usize {
    assignments.iter()
        .filter(|assignment| is_subrange(assignment.0, assignment.1) || is_subrange(assignment.1, assignment.0))
        .count()
}

fn solve_part2(assignments: &Vec<(Range, Range)>) -> usize {
    assignments.iter()
        .filter(|assignment| overlaps(assignment.0, assignment.1))
        .count()
}
