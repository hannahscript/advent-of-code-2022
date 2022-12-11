use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Operand {
    OLD,
    LITERAL(u64)
}

#[derive(Debug, Clone)]
enum Operator {
    PLUS,
    TIMES
}

#[derive(Debug, Clone)]
struct Operation {
    left: Operand,
    right: Operand,
    operator: Operator
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    when_true: usize,
    when_false: usize,
    total_inspections: u64
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: vec![],
            operation: Operation {
                left: Operand::OLD,
                right: Operand::OLD,
                operator: Operator::PLUS
            },
            test: 0,
            when_true: 0,
            when_false: 0,
            total_inspections: 0
        }
    }
}

fn read_input() -> Vec<Monkey> {
    let mut m = Monkey::new();

    let contents = match read_to_string("inputs/day11.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");
    
    let mut result: Vec<Monkey> = vec![];
    for line in lines {
        if line.starts_with("Monkey") {
            m = Monkey::new();
            let (_, id) = line.split_at(7);
            m.id = id[..1].parse().unwrap();
        } else if line.contains("Starting items") {
            let (_, items_raw) = line.split_at(18);
            m.items = items_raw.split(", ").into_iter().map(|i| i.parse().unwrap()).collect();
        } else if line.contains("Operation") {
            let (_, op_raw) = line.split_at(19);
            m.operation = parse_op(op_raw);
        } else if line.contains("Test") {
            let (_, div) = line.split_at(21);
            m.test = div.parse().unwrap();
        } else if line.contains("If true") {
            let (_, id) = line.split_at(29);
            m.when_true = id.parse().unwrap();
        } else if line.contains("If false") {
            let (_, id) = line.split_at(30);
            m.when_false = id.parse().unwrap();
            result.push(m.clone());
        }
    }
    
    result
}

fn parse_op(text: &str) -> Operation {
    let op = if text.contains("*") { "*" } else { "+" };
    let ops: Vec<Operand> = text.split(op).into_iter().map(|o: &str| 
        match o.trim().parse() {
           Err(_) => Operand::OLD,
            Ok(n) => Operand::LITERAL(n)
        }
    ).collect();
    
    Operation {
        left: ops[0].clone(),
        right: ops[1].clone(),
        operator: if op == "+" { Operator::PLUS } else { Operator::TIMES }
    }
}

fn apply_m(worry: &u64, op: &Operation, m: u64) -> u64 {
    let left = match &op.left {
        Operand::OLD => worry,
        Operand::LITERAL(n) => n
    };
    let right = match &op.right {
        Operand::OLD => worry,
        Operand::LITERAL(n) => n
    };
    match op.operator {
        Operator::PLUS => (left + right) % m,
        Operator::TIMES => (left * right) % m
    }
}

fn apply_s(worry: &u64, op: &Operation) -> u64 {
    let left = match &op.left {
        Operand::OLD => worry,
        Operand::LITERAL(n) => n
    };
    let right = match &op.right {
        Operand::OLD => worry,
        Operand::LITERAL(n) => n
    };
    match op.operator {
        Operator::PLUS => (left + right) / 3,
        Operator::TIMES => (left * right) / 3
    }
}

fn monkey_around<F>(id: usize, monkeys: &mut Vec<Monkey>, apply: F)
    where F: Fn(&u64, &Operation) -> u64 {
    let mut transfers: Vec<(usize, u64)> = vec![];
    {
        let monkey = &mut monkeys[id];
        
        for item in &monkey.items {
            monkey.total_inspections += 1;
            let worry = apply(item, &monkey.operation);
            
            if worry % monkey.test == 0 {
                transfers.push((monkey.when_true, worry));
            } else {
                transfers.push((monkey.when_false, worry));
            }
        }

        monkey.items.clear();
    }
    
    for (other_id, worry) in transfers {
        monkeys[other_id].items.push(worry);
    }
}

pub fn solve() {
    let mut input = read_input();
    let part1 = solve_part1(&mut input.clone());
    let part2 = solve_part2(&mut input);

    println!("Day 11");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn simulate<F>(rounds: u32, monkeys: &mut Vec<Monkey>, apply: F) -> u64
    where F: Fn(&u64, &Operation) -> u64 {
    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            monkey_around(id, monkeys, &apply);
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.total_inspections).collect();
    inspections.sort();
    inspections[inspections.len() - 2 ..].iter().product()
    
}

fn solve_part1(monkeys: &mut Vec<Monkey>) -> u64 {
    simulate(20, monkeys, apply_s)
}

fn solve_part2(monkeys: &mut Vec<Monkey>) -> u64 {
    let m = monkeys.iter().map(|m| m.test).product();
    simulate(10000, monkeys, |worry, op| apply_m(worry, op, m))
}
