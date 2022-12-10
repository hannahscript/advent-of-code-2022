use crate::utility::utils::parse_file;

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i32)
}

#[derive(Debug)]
struct State {
    ip: usize,
    reg_x: i32
}

fn read_input() -> Vec<Instruction> {
    parse_file("inputs/day10.txt", |line| {
        let split = line.split_once(" ");
        match split {
            None => {
                match line {
                    "noop" => Instruction::NOOP,
                    _ => panic!("Unknown instruction: {}", line)
                }
            }
            Some((int, arg)) => {
                let n: i32 = arg.parse().unwrap();
                match int {
                    "addx" => Instruction::ADDX(n),
                    _ => panic!("Unknown instruction: {}", line)
                }
            }
        }
    })
}

fn add_noop_cycles(input: &Vec<Instruction>) -> Vec<Instruction> {
    input.iter().flat_map(|int| {
        match int {
            Instruction::NOOP => vec![Instruction::NOOP],
            Instruction::ADDX(n) => vec![Instruction::NOOP, Instruction::ADDX(*n)]
        }
    }).collect()
}

pub fn solve() {
    let mut input = read_input();
    input = add_noop_cycles(&input);
    let part1 = solve_part1(&input);

    println!("Day 10");
    println!("Part 1: {}", part1);
    println!("Part 2:");
    solve_part2(&input);
}

fn simulate(state: &mut State, instructions: &Vec<Instruction>, simulate: usize) -> i32 {
    for ip in state.ip..state.ip+simulate {
        let int = &instructions[ip];
        match int {
            Instruction::NOOP => {}
            Instruction::ADDX(n) => state.reg_x += *n
        }
    }
    
    state.ip = state.ip + simulate;
    get_signal_strength(state)
}

fn get_signal_strength(state: &State) -> i32 {
    state.reg_x * ( state.ip as i32 + 1 )
}

fn solve_part1(instructions: &Vec<Instruction>) -> i32 {
    let mut state = State { ip: 0, reg_x: 1 };
    let mut sum = 0;
    
    sum += simulate(&mut state, &instructions, 19);
    sum += simulate(&mut state, &instructions, 40);
    sum += simulate(&mut state, &instructions, 40);
    sum += simulate(&mut state, &instructions, 40);
    sum += simulate(&mut state, &instructions, 40);
    sum += simulate(&mut state, &instructions, 40);
    
    
    sum
}

fn solve_part2(instructions: &Vec<Instruction>) {
    let mut state = State { ip: 0, reg_x: 1 };

    for _ in 0..6 {
        for x in 0..40 {
            if x == state.reg_x - 1 || x == state.reg_x  || x == state.reg_x + 1  {
                print!("⚪");
            } else {
                print!("⚫");
            }
            simulate(&mut state, &instructions, 1);
        }
        println!();
    }
}
