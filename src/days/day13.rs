use std::cell::RefCell;
use std::cmp::{min, Ordering};
use std::rc::Rc;
use crate::utility::utils::parse_file;

#[derive(Debug, Clone)]
enum Tree {
    Node(Rc<RefCell<Vec<Tree>>>),
    Leaf(i32)
}

fn read_input() -> Vec<Tree> {
    parse_file("inputs/day13.txt", |line| {
        let parsed = parse_tree(&mut line.chars().collect());
        let unparsed = unparse_tree(&parsed);
        
        if line != unparsed {
            panic!("Fucked up {} !== {}", line, unparsed);
        }
        
        parsed
    })
}

fn parse_tree(tokens: &mut Vec<char>) -> Tree {
    let c = tokens.remove(0);
    if c == '[' {
        let mut contents: Vec<Tree> = vec![];
        let mut peek = tokens[0];
        while peek != ']' {
            let next = parse_tree(tokens);
            contents.push(next);
            if tokens[0] == ',' {
                tokens.remove(0);
            }
            peek = tokens[0];
        }
        tokens.remove(0);
        
        Tree::Node(Rc::new(RefCell::new(contents)))
    } else {
        let mut num_str = c.to_string();
        while tokens[0] != ',' && tokens[0] != ']' {
            num_str.push_str(&tokens.remove(0).to_string());
        }
        Tree::Leaf(num_str.parse().unwrap())
    }
}

fn unparse_tree(tree: &Tree) -> String {
    match tree {
        Tree::Node(children) => {
            let c: Vec<String> = children.borrow().iter().map(|t| unparse_tree(t)).collect();
            let mut result = "[".to_string();
            result.push_str(&c.join(","));
            result.push_str("]");
            result
        },
        Tree::Leaf(n) => n.to_string()
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == 0
    }
}

impl Eq for Tree {}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match compare(self, other) {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            n => panic!("Comparison result incorrect: {}", n)
        })
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// -1 right order, 0 neutral, 1 wrong order
fn compare(tree_a: &Tree, tree_b: &Tree) -> i32 {
    //println!("Compare {} to {}", unparse_tree(tree_a), unparse_tree(tree_b));
    match tree_a {
        Tree::Node(children_a) => {
            let ba = &children_a.borrow();
            match tree_b {
                Tree::Node(children_b) => {
                    let bb = &children_b.borrow();
                    for i in 0..min(ba.len(), bb.len()) {
                        let r = compare(&ba[i], &bb[i]);
                        if r != 0 {
                            return r;
                        }
                    }

                    (ba.len() as i32 - bb.len() as i32).signum()
                }
                Tree::Leaf(b) => {
                    if ba.is_empty() {
                        -1
                    } else {
                        let r = compare(&ba[0], tree_b);
                        if r == 0 {
                            (ba.len() as i32 - 1).signum()
                        } else {
                            r
                        }
                    }
                }
            }
        }
        Tree::Leaf(a) => {
            match tree_b {
                Tree::Node(children_b) => {
                    let bb = &children_b.borrow();
                    if bb.is_empty() {
                        1
                    } else {
                        let r = compare(tree_a, &bb[0]);
                        if r == 0 {
                            (1 - bb.len() as i32).signum()
                        } else {
                            r
                        }
                    }
                }
                Tree::Leaf(b) => (a - b).signum()
            }
        }
    }
}

pub fn solve() {
    let mut input = read_input();
    
    //println!("{:?}", input);
    
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    
    println!("Day 13");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &Vec<Tree>) -> usize {
    let mut sum = 0;
    for i in (0..input.len() - 1).step_by(2) {
        // println!("Comparing pair {}: {} to {}", i / 2 + 1, i, i+ 1);
        // println!("{:?}", input[i]);
        // println!("{:?}", input[i+1]);
        if compare(&input[i], &input[i+1]) < 0 {
            //println!("Pair {} is in order", i / 2 + 1);
            sum += i / 2 + 1;
        }
    }
    
    sum
}

fn solve_part2(input: &Vec<Tree>) -> usize {
    let mut list = (*input).clone();
    let div_1 = parse_tree(&mut "[[2]]".chars().collect());
    let div_2 = parse_tree(&mut "[[6]]".chars().collect());
    list.push(div_1.clone());
    list.push(div_2.clone());

    list.sort();

    (1 + list.binary_search(&div_1).unwrap()) * (1 + list.binary_search(&div_2).unwrap())
}
