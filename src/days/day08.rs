use crate::utility::utils::parse_file;

fn read_input() -> Vec<Vec<u32>> {
    parse_file("inputs/day08.txt", |line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    })
}

pub fn solve() {
    let input = read_input();

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 08");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(trees: &Vec<Vec<u32>>) -> usize {
    let dim = trees.len();
    let mut vis = vec![vec![0; dim]; dim];
    let mut biggest_tree;
    
    for y in 1..dim-1 {
        biggest_tree = trees[y][0];
        for x in 1..dim-1 {
            let tree = trees[y][x];
            if tree > biggest_tree {
                biggest_tree = tree;
                vis[y][x] += 1;
            }
        }

        biggest_tree = trees[y][dim - 1];
        for x in (1..dim-1).rev() {
            let tree = trees[y][x];
            if tree > biggest_tree {
                biggest_tree = tree;
                vis[y][x] += 1;
            }
        }
    }

    for x in 1..dim-1 {
        biggest_tree = trees[0][x];
        for y in 1..dim-1 {
            let tree = trees[y][x];
            if tree > biggest_tree {
                biggest_tree = tree;
                vis[y][x] += 1;
            }
        }

        biggest_tree = trees[dim - 1][x];
        for y in (1..dim-1).rev() {
            let tree = trees[y][x];
            if tree > biggest_tree {
                biggest_tree = tree;
                vis[y][x] += 1;
            }
        }
    }
    
    vis.iter().map(|row| row.into_iter().filter(|c| **c > 0).count()).sum::<usize>() + 2 * dim + 2 * (dim - 2)
}

fn get_view_dist(trees: &Vec<Vec<u32>>, x_start: usize, y_start: usize) -> usize {
    let dim = trees.len();
    let height = trees[y_start][x_start];
    let mut score = 1;
    
    let mut dist = 0;
    for x in x_start+1..dim {
        dist = x - x_start;
        if trees[y_start][x] >= height {
            break;
        }
    }
    score *= dist;
    
    dist = 0;
    for x in (0..x_start).rev() {
        dist = x_start - x;
        if trees[y_start][x] >= height {
            break;
        }
    }
    score *= dist;

    dist = 0;
    for y in y_start+1..dim {
        dist = y - y_start;
        if trees[y][x_start] >= height {
            break;
        }
    }
    score *= dist;

    dist = 0;
    for y in (0..y_start).rev() {
        dist = y_start - y;
        if trees[y][x_start] >= height {
            break;
        }
    }
    score *= dist;
    
    score
}

fn solve_part2(trees: &Vec<Vec<u32>>) -> usize {
    let dim = trees.len();
    let mut scores = vec![vec![1; dim]; dim];

    for y in 1..dim-1 {
        for x in 1..dim-1 {
            scores[y][x] = get_view_dist(&trees, x, y);
        }
    }
    
    *scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap()
}
