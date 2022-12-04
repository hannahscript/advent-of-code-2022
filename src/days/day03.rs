use std::collections::{HashMap, HashSet};
use std::fs::{File};
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<String> {
    let input = match File::open("inputs/day03.txt") {
        Ok(contents) => contents,
        Err(err) => panic!("Can't open file: {:?}", err)
    };
    let reader = BufReader::new(input);

    let mut result: Vec<String> = Vec::new();
    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => result.push(line),
            Err(err) => panic!("Can't read line: {:?}", err)
        }
    }

    result
}

pub fn solve() {
    let input = read_input();
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 03");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(rucksacks: &Vec<String>) -> u32 {
    rucksacks.iter().map(|rs| {get_priority(rs)}).sum()
}

fn get_priority(rucksack: &String) -> u32 {
    let (compartment_a, compartment_b) = rucksack.split_at(rucksack.len() / 2);
    let shared_letter = get_shared_letter(compartment_a, compartment_b);
    get_letter_priority(shared_letter)
}

fn get_letter_priority(letter: char) -> u32 {
    if letter.is_ascii_uppercase() {
        letter as u32 - 65 + 27
    } else {
        letter as u32 - 97 + 1
    }
}

fn get_shared_letter(c1: &str, c2: &str) -> char {
    let mut letters_c1 = HashSet::new();
    for letter in c1.chars() {
        letters_c1.insert(letter);
    }

    for letter in c2.chars() {
        if letters_c1.contains(&letter) {
            return letter;
        }
    }

    panic!("Compartments don't share items: {} / {}", c1, c2);
}

fn solve_part2(rucksacks: &Vec<String>) -> u32 {
    let mut count: HashMap<char, u32> = HashMap::new();
    let mut i = 0;
    let mut sum = 0;
    let mut badge;

    for rucksack in rucksacks {
        badge = count_letters(rucksack, &mut count);

        if i % 3 == 2 {
            count.clear();
            sum += get_letter_priority(badge);
        }

        i += 1;
    }

    sum
}

fn count_letters(rucksack: &String, map: &mut HashMap<char, u32>) -> char {
    let mut max_key = '?';
    let mut max_val = 0;
    let mut already_counted: HashSet<char> = HashSet::new();

    for letter in rucksack.chars() {
        match map.get(&letter) {
            None => {
                map.insert(letter, 1);
            },
            Some(count) => {
                if already_counted.contains(&letter) {
                    continue;
                }
                let new_count = count + 1;
                map.insert(letter, new_count);
                if new_count > max_val {
                    max_key = letter;
                    max_val = new_count;
                }
            }
        }
        already_counted.insert(letter);
    }

    max_key
}
