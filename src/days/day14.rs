use std::cmp::{max, min};
use std::fmt::{ Display, Formatter};
use std::time::SystemTime;
use crate::utility::utils::parse_file;

fn read_input() -> Vec<Vec<(usize, usize)>> {
    parse_file("inputs/day14.txt", |line| {
        line.split("->").into_iter().map(|pair| {
            let (from, to) = pair.trim().split_once(",").unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        }).collect()
    })
}

#[derive(Clone, PartialEq)]
enum Tile {
    AIR, ROCK, SAND
}

#[derive(Clone)]
struct Sandbox {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    drop_x: usize,
    fast_pos: Option<(usize, usize)>
}

impl Sandbox {
    fn new(max_y: usize) -> Self {
        let height = max_y + 3;
        let width = height * 2 - 1 + 2;
        let offset = 500 - width / 2;
        Sandbox {
            tiles: vec![Tile::AIR; width * height],
            width,
            height,
            drop_x: 500 - offset,
            fast_pos: None
        }
    }
    
    fn put(self: &mut Self, point: (usize, usize), tile: Tile) {
        self.tiles[point.1 * self.width + point.0] = tile;
    }

    fn get(self: &Self, point: (usize, usize)) -> &Tile {
        if point.1 == self.height - 1 {
            return &Tile::ROCK;
        }
        &self.tiles[point.1 * self.width + point.0]
    }
    
    fn draw_line(self: &mut Self, from: (usize, usize), to: (usize, usize)) {
        if from.0 == to.0 {
            for y in min(from.1, to.1)..max(from.1, to.1)+1 {
                self.put((from.0, y), Tile::ROCK);
            }
        } else if from.1 == to.1 {
            for x in min(from.0, to.0)..max(from.0, to.0)+1 {
                self.put((x, from.1), Tile::ROCK);
            }
        } else {
            panic!("Diagonal line: {:?} to {:?}", from, to);
        }
    }

    fn place_sand(self: &mut Self) -> (usize, usize) {
        let mut sand = if self.fast_pos.is_none() {
            (self.drop_x, 0)
        } else {
            self.fast_pos.unwrap()
        };
        let mut prev_pos = None;
        loop {
            if *self.get((sand.0, sand.1 + 1)) == Tile::AIR {
                prev_pos = Some(sand);
                sand.1 += 1;
            } else if sand.0 > 0 && *self.get((sand.0 - 1, sand.1 + 1)) == Tile::AIR {
                prev_pos = Some(sand);
                sand.0 -= 1;
                sand.1 += 1;
            } else if sand.0 < self.width - 1 && *self.get((sand.0 + 1, sand.1 + 1)) == Tile::AIR {
                prev_pos = Some(sand);
                sand.0 += 1;
                sand.1 += 1;
            } else {
                self.fast_pos = prev_pos;
                self.put(sand, Tile::SAND);
                return sand;
            }
        }
    }
}

impl Display for Sandbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}\n", self.width, self.height).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", match self.get((x, y)) {
                    Tile::AIR => ".",
                    Tile::ROCK => "█",
                    Tile::SAND => "S"
                }).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

fn create_sandbox(lines: &Vec<Vec<(usize, usize)>>) -> Sandbox {
    let mut min_x: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for line in lines {
        for point in line {
            min_x = min(min_x, point.0);
            max_x = max(max_x, point.0);
            max_y = max(max_y, point.1);
        }
    }

    let height = max_y + 3;
    let width = height * 2 - 1 + 2;
    let offset = 500 - width / 2;
    let mut sandbox = Sandbox {
        tiles: vec![Tile::AIR; width * height],
        width,
        height,
        drop_x: 500 - offset,
        fast_pos: None
    };

    for line in lines {
        for i in 0..line.len() - 1 {
            let from = (line[i].0 - offset, line[i].1);
            let to = (line[i+1].0 - offset, line[i+1].1);
            sandbox.draw_line(from, to);
        }
    }
    
    sandbox
}

pub fn solve() {
    let elapsed = SystemTime::now();
    
    let mut input = read_input();
    let mut sandbox = create_sandbox(&input);

    
    let part1 = solve_part1(&mut sandbox.clone());
    let part2 = solve_part2(&mut sandbox.clone());
    
    println!("Elapsed: {}ms", elapsed.elapsed().unwrap().as_millis());

    println!("Day 14");
    println!("Part 1: {}", part1); // 1072
    println!("Part 2: {}", part2); // 24659
}

fn solve_part1(sandbox: &mut Sandbox) -> usize {
    let mut i = 0;
    while sandbox.place_sand().1 <= sandbox.height - 3 {
        i += 1;
    }
    
    i 
}

fn solve_part2(sandbox: &mut Sandbox) -> usize {
    let mut i = 0;
    while sandbox.place_sand() != (sandbox.drop_x, 0) {
        i += 1;
    }

    i + 1
}
