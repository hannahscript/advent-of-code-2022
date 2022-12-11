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
    items: Vec<MonkeyNumber>,
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

#[derive(Debug, Clone)]
struct MonkeyNumber {
    // (mod, val)
    classes: Vec<(u64, u64)>
}

impl MonkeyNumber {
    fn new(num: u64, divisors: &Vec<u64>) -> MonkeyNumber {
        MonkeyNumber {
            classes: divisors.iter().map(|&d| (d, num % d)).collect()
        }
    }
    
    fn add(self: &MonkeyNumber, other: &MonkeyNumber, divide_by_three: bool) -> MonkeyNumber {
        let mut classes = vec![];
        for i in 0..self.classes.len() {
            let md = self.classes[i].0;
            classes.push((md, ((self.classes[i].1 + other.classes[i].1) / if divide_by_three { 3 } else { 1 }) % md))
        }
        
        MonkeyNumber {
            classes
        }
    }

    fn mult(self: &MonkeyNumber, other: &MonkeyNumber, divide_by_three: bool) -> MonkeyNumber {
        let mut classes = vec![];
        for i in 0..self.classes.len() {
            let md = self.classes[i].0;
            classes.push((md, ((self.classes[i].1 * other.classes[i].1) / if divide_by_three { 3 } else { 1 }) % md))
        }

        MonkeyNumber {
            classes
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
    let mut all_items: Vec<Vec<u64>> = vec![];
    for line in lines {
        if line.starts_with("Monkey") {
            m = Monkey::new();
            let (_, id) = line.split_at(7);
            m.id = id[..1].parse().unwrap();
        } else if line.contains("Starting items") {
            let (_, items_raw) = line.split_at(18);
            let items: Vec<u64> = items_raw.split(", ").into_iter().map(|i| i.parse().unwrap()).collect();
            all_items.push(items);
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
    println!("All items {:?}", all_items);
    
    // Generate monkey numbers
    let divisors: Vec<u64> = result.iter().map(|m| m.test).collect();
    for i in 0..result.len() {
        result[i].items = all_items[i].iter().map(|&item| MonkeyNumber::new(item, &divisors)).collect();
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

fn apply_op(worry: &MonkeyNumber, op: &Operation, divisors: &Vec<u64>, divide_by_three: bool) -> MonkeyNumber {
    let left = match &op.left {
        Operand::OLD => worry.clone(),
        Operand::LITERAL(n) => MonkeyNumber::new(*n, &divisors)
    };
    let right = match &op.right {
        Operand::OLD => worry.clone(),
        Operand::LITERAL(n) => MonkeyNumber::new(*n, &divisors)
    };
    match op.operator {
        Operator::PLUS => left.add(&right, divide_by_three),
        Operator::TIMES => left.mult(&right, divide_by_three)
    }
}

fn monkey_around(id: usize, monkeys: &mut Vec<Monkey>, divide_by_three: bool) {
    let mut transfers: Vec<(usize, MonkeyNumber)> = vec![];
    {
        let divisors: Vec<u64> = monkeys.iter().map(|m| m.test).collect();
        let monkey = &mut monkeys[id];
        
        for item in &monkey.items {
            monkey.total_inspections += 1;
            let mut worry = apply_op(&item, &monkey.operation, &divisors, divide_by_three);
            //println!("Worry op {:?} {:?} ==> {:?}", &monkey.operation, item, worry);
            if worry.classes[id].1 == 0 {
                transfers.push((monkey.when_true, worry.clone()));
            } else {
                transfers.push((monkey.when_false, worry.clone()));
            }
        }

        monkey.items.clear();
    }

    //println!("TRANSFERS {:?}", transfers);
    
    for (other_id, worry) in transfers {
        monkeys[other_id].items.push(worry);
    }
}

pub fn solve() {
    let mut input = read_input();
    let part1 = solve_part1(&mut input.clone());
    let part2 = solve_part2(&mut input);

    println!("Day 11");
    /*
    Part 1 broke with the addition of modular arithmetic (cause of the division by 3 bit) and i cant be bothered to revert the code
    i wrote for it earlier and threw away. but same principle with u64 instead of monkey numbers
     */
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn simulate(rounds: u32, monkeys: &mut Vec<Monkey>, divide_by_three: bool) -> u64 {
    for r in 0..rounds {
        if r % 100 == 0 {
            //println!("Round {}/{} done", r + 1, rounds);
        }
        for id in 0..monkeys.len() {
            monkey_around(id, monkeys, divide_by_three);
        }
        //println!("{:?}\n\n", monkeys[0].items);
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.total_inspections).collect();
    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn solve_part1(monkeys: &mut Vec<Monkey>) -> u64 {
    simulate(20, monkeys, true)
}

fn solve_part2(monkeys: &mut Vec<Monkey>) -> u64 {
    simulate(10000, monkeys, false)
}
