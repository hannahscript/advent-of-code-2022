use std::fs::read_to_string;

fn read_input_day_01() -> Vec<Vec<i32>> {
    let contents = match read_to_string("inputs/day01.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");

    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut vec: Vec<i32> = Vec::new();
    for line in lines {
        if line.len() > 0 {
            vec.push(line.parse::<i32>().unwrap());
        } else {
            result.push(vec.clone());
            vec.clear();
        }
    }

    result
}

pub fn solve() {
    let input = read_input_day_01();
    let mut calories = get_calories(input);
    calories.sort();
    let part1 = day_01_part1(&calories);
    let part2 = day_01_part2(&calories);

    println!("Day 01");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_calories(input: Vec<Vec<i32>>) -> Vec<i32> {
    input.into_iter().map(|elf| {
        elf.into_iter().reduce(|total, calories| { total + calories }).unwrap()
    }).collect()
}

fn day_01_part1(calories: &Vec<i32>) -> i32 {
    *calories.into_iter().max().unwrap()
}

fn day_01_part2(calories: &Vec<i32>) -> i32 {
    let len = calories.len();
    calories[len - 1] + calories[len - 2] + calories[len - 3]
}
