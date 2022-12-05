use std::fs::read_to_string;
use regex::{Captures, Regex};

#[derive(Debug)]
#[derive(Clone)]
struct StackGame {
    stacks: Vec<Vec<String>>,
    instructions: Vec<(usize, usize, usize)>
}

fn read_input() -> StackGame {
    let stack_rgx = Regex::new(r"\[(\w)]|\s(\s{3})").unwrap();
    let move_rgx = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut captures: Vec<Captures> = Vec::new();

    let contents = match read_to_string("inputs/day05.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");
    
    let mut result = StackGame {
        stacks: Vec::new(),
        instructions: Vec::new()
    };
    for line in lines {
        if line.starts_with(" 1") {
            let stack_amount = line.split_whitespace().last().unwrap().parse::<usize>().unwrap();
            captures.reverse();
            add_stacks(&captures, &mut result, stack_amount);
            captures.clear();
        } else if line.starts_with("move") {
            for c in move_rgx.captures_iter(line) {
                captures.push(c);
            }
        } else if line.is_empty() {
            if !captures.is_empty() {
                add_moves(&captures, &mut result);
            }
        } else {
            for c in stack_rgx.captures_iter(line) {
                captures.push(c);
            }
        }
    }
    
    result
}

fn add_stacks(captures: &Vec<Captures>, stack_game: &mut StackGame, stack_amount: usize) {
    for _ in 0..stack_amount {
        stack_game.stacks.push(Vec::new());
    }
    
    let mut si = 0;
    for c in captures {
        let m = c.get(1).map_or("", |m| m.as_str());
        if !m.is_empty() {
            stack_game.stacks.get_mut(stack_amount - 1 - (si % stack_amount)).unwrap().push(m.to_string())
        }
        
        si += 1;
    }
}

fn add_moves(captures: &Vec<Captures>, stack_game: &mut StackGame) {
    for c in captures {
        let x1 = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let x2 = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let x3 = c.get(3).unwrap().as_str().parse::<usize>().unwrap();
        stack_game.instructions.push((x1, x2, x3));
    }
}

pub fn solve() {
    let mut input = read_input();

    let part1 = solve_part1(&mut input.clone());
    let part2 = solve_part2(&mut input);
    
    println!("Day 05");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(stack_game: &mut StackGame) -> String {
    for ins in stack_game.instructions.iter() {
        let (amount, from, to) = *ins;
        for _ in 0..amount {
            let crt = stack_game.stacks.get_mut(from - 1).unwrap().pop().unwrap();
            stack_game.stacks.get_mut(to - 1).unwrap().push(crt);
        }
    }
    
    let mut result= String::new();
    for stack in stack_game.stacks.iter() {
        result.push_str(stack.last().unwrap());
    }
    
    result
}

fn solve_part2(stack_game: &mut StackGame) -> String {
    let mut temp_stack: Vec<String> = Vec::new();
    for ins in stack_game.instructions.iter() {
        let (amount, from, to) = *ins;
        for _ in 0..amount {
            let crt = stack_game.stacks.get_mut(from - 1).unwrap().pop().unwrap();
            temp_stack.push(crt);
        }
        while !temp_stack.is_empty() {
            let crt = temp_stack.pop().unwrap();
            stack_game.stacks.get_mut(to - 1).unwrap().push(crt);
        }
    }

    let mut result= String::new();
    for stack in stack_game.stacks.iter() {
        result.push_str(stack.last().unwrap());
    }

    result
}
