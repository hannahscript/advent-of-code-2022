use std::fs::read_to_string;

fn read_input() -> Vec<(i32, i32)> {
    let contents = match read_to_string("inputs/day02.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");

    let mut result: Vec<(i32, i32)> = Vec::new();
    for line in lines {
        if line.is_empty() {continue};
        let pair_raw: Vec<&str> = line.split(" ").collect();
        let pair = (map_move(pair_raw[0]), map_move(pair_raw[1]));
        result.push(pair);
    }

    result
}

fn map_move(mov: &str) -> i32 {
    match mov {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("Unexpected move: {}", mov)
    }
}

pub fn solve() {
    let input = read_input();
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 02");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(moves: &Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for mov in moves {
        let round_score = get_score(mov);
        score += round_score + mov.1 + 1;
    }
    score
}

fn solve_part2(guides: &Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for guide in guides {
        let my_move = get_move(guide);
        let round_score = get_score(&(guide.0, my_move));
        score += round_score + my_move + 1;
    }
    score
}

fn get_score(mov: &(i32, i32)) -> i32 {
    if mov.0 == mov.1 {
        3
    } else if mov.1 == (mov.0 + 1).rem_euclid(3) {
        6
    } else {
        0
    }
}

fn get_move(guide: &(i32, i32)) -> i32 {
    match guide.1 {
        0 => (guide.0 - 1).rem_euclid(3), // Lose
        1 => guide.0,                         // Draw
        2 => (guide.0 + 1).rem_euclid(3), // Win
        _ => panic!("Unknown strategy {}", guide.1)
    }
}
