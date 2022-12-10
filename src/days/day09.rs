use std::borrow::BorrowMut;
use std::collections::HashSet;
use crate::utility::utils::parse_file;

#[derive(Debug)]
enum Move {
    UP(u32),
    DOWN(u32),
    LEFT(u32),
    RIGHT(u32)
}

#[derive(Debug)]
struct Rope {
    head: (i32, i32),
    knots: Vec<(i32, i32)>
}

fn read_input() -> Vec<Move> {
    parse_file("inputs/day09.txt", |line| {
        let (c, n) = line.split_at(2);
        let distance: u32 = n.parse().unwrap();
        match c {
            "U " => Move::UP(distance),
            "D " => Move::DOWN(distance),
            "L " => Move::LEFT(distance),
            "R " => Move::RIGHT(distance),
            _ => panic!("Unknown move: {}", c)
        }
    })
}

pub fn solve() {
    let input = read_input();
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 09");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn move_knot(leading_knot: &(i32, i32), moving_knot: &(i32, i32)) -> (i32, i32) {
    let dx = leading_knot.0 - moving_knot.0;
    let dy = leading_knot.1 - moving_knot.1;

    let tail_dx = dx - dx.signum() * (dx.abs() >= dy.abs()) as i32;
    let tail_dy = dy - dy.signum() * (dy.abs() >= dx.abs()) as i32;

    (moving_knot.0 + tail_dx, moving_knot.1 + tail_dy)
}

fn nudge_rope(rope: &mut Rope, x_dir: i32, y_dir: i32, visited: &mut HashSet<(i32, i32)>) {
    rope.head = (rope.head.0 + x_dir, rope.head.1 + y_dir);

    let mut leading_knot = rope.head;
    for i in 0..rope.knots.len() {
        let current_knot = rope.knots[i].borrow_mut();
        let new_knot_pos = move_knot(&leading_knot, current_knot);

        current_knot.0 = new_knot_pos.0;
        current_knot.1 = new_knot_pos.1;
        
        leading_knot = new_knot_pos;
    }

    visited.insert(rope.knots.last().unwrap().clone());
}

fn move_rope(rope: &mut Rope, mov: &Move, visited: &mut HashSet<(i32, i32)>) {
    match mov {
        Move::UP(n) => {
            for _ in 0..*n {
                nudge_rope(rope, 0, 1, visited);
            }
        }
        Move::DOWN(n) => {
            for _ in 0..*n {
                nudge_rope(rope, 0, -1, visited);
            }
        }
        Move::LEFT(n) => {
            for _ in 0..*n {
                nudge_rope(rope, -1, 0, visited);
            }
        }
        Move::RIGHT(n) => {
            for _ in 0..*n {
                nudge_rope(rope, 1, 0, visited);
            }
        }
    }
}

fn solve_part1(moves: &Vec<Move>) -> usize {
    let mut rope = Rope {head: (0, 0), knots: vec![(0, 0); 1]};
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for mov in moves {
        move_rope(&mut rope, mov, &mut visited);
    }
    
    visited.len()
}

fn solve_part2(moves: &Vec<Move>) -> usize {
    let mut rope = Rope {head: (0, 0), knots: vec![(0, 0); 9]};
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for mov in moves {
        move_rope(&mut rope, mov, &mut visited);
    }

    visited.len()
}
