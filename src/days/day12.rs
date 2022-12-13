use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::thread::current;
use std::time::SystemTime;

struct GridGraph {
    width: i32,
    height: i32,
    grid: Vec<i32>
}

impl GridGraph {
    fn new(    width: i32,
               height: i32, grid: Vec<i32>) -> Self {
        GridGraph { width, height, grid }
    }
    
    fn get_neighbours(self: &Self, center: (i32, i32)) -> Vec<i32> {
        let mut result = vec![];
        for (x, y) in [(center.0 +1, center.1 ), (center.0 -1,center.1 ), (center.0, center.1 +1), (center.0 ,center.1 -1)] {
            if x >= 0 && x < self.width && y >= 0 && y < self.height {
                result.push(self.grid[(y * self.width + x) as usize]);
            }
        }
        
        result
    }
}

#[derive(Debug, Clone)]
struct Input {
    elevations: Vec<Vec<u32>>,
    start: (usize, usize),
    end: (usize, usize)
}

#[derive(Debug, Clone)]
struct Graph {
    width: usize,
    height: usize,
    adjacency: Vec<Vec<Option<i32>>>
}

#[derive(Debug, Clone)]
struct GraphSearch {
    graph: Graph,
    working: Vec<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    costs: HashMap<(usize, usize), u32>
}

#[derive(Debug, Clone)]
struct BellmanFord {
    graph: Graph,
    distance: HashMap<(usize, usize), i32>,
    pred: HashMap<(usize, usize), (usize, usize)>
}

#[derive(Debug, Clone)]
struct AStar {
    graph: Graph,
    start_node: (usize, usize),
    cost: HashMap<(usize, usize), i32>,
    h_cost: HashMap<(usize, usize), i32>,
    pred: HashMap<(usize, usize), (usize, usize)>,
    open: HashSet<(usize, usize)>
}

impl AStar {
    fn new(graph: Graph, start_node: (usize, usize)) -> Self {
        AStar {
            graph,
            start_node,
            cost: HashMap::new(),
            h_cost: HashMap::new(),
            pred: HashMap::new(),
            open: HashSet::new()
        }
    }
    
    fn initialize(self: &mut Self, start_node: (usize, usize)) {
        self.cost.clear();
        self.h_cost.clear();
        self.pred.clear();
        self.open.clear();
        self.start_node = start_node;
        for y in 0..self.graph.height {
            for x in 0..self.graph.width {
                self.cost.insert((x, y), (self.graph.height * self.graph.width + 99999) as i32);
                self.h_cost.insert((x, y), (self.graph.height * self.graph.width + 99999) as i32);
            }
        }
        self.cost.insert(start_node, 0);
        self.h_cost.insert(start_node, AStar::heuristic(&start_node, &start_node));
        self.open.insert(start_node);
    }
    
    fn get_edge_cost(self: &mut Self, from: &(usize, usize), to: &(usize, usize)) -> i32 {
        self.graph.adjacency[self.graph.width * from.1 + from.0][self.graph.width * to.1 + to.0].unwrap()
    }
    
    fn heuristic(start_node: &(usize, usize), node: &(usize, usize)) -> i32 {
        (start_node.0 as i32 - node.0 as i32).abs() + (start_node.1 as i32 - node.1 as i32).abs()
    }

    fn find(self: &mut Self, target_node: (usize, usize)) -> Result<i32, String> {
        while !self.open.is_empty() {
            //println!("Open set size {}", self.open.len());
            let current: (usize, usize) = *self.open.iter()
                .min_by(|a, b| self.h_cost[*a].cmp(&self.h_cost[*b]))
                .unwrap();
            //println!("Self.open {:?} candiate = {}", current, self.h_cost[&current]);

            if current == target_node {
                return Ok(self.get_steps(target_node));
            }
            
            self.open.remove(&current);
            
            let nbs = self.get_neighbours(current);
            for nb in nbs {
                let score = self.cost[&current] + self.get_edge_cost(&current, &nb);
                if score < self.cost[&nb] {
                    self.pred.insert(nb, current);
                    self.cost.insert(nb, score);
                    self.h_cost.insert(nb, score + AStar::heuristic(&self.start_node, &nb));
                    if !self.open.contains(&nb) {
                        self.open.insert(nb);
                    }
                }
            }
        }

        return Err("No Path".to_string());
    }

    fn get_neighbours(self: &mut Self, node: (usize, usize)) -> Vec<(usize, usize)> {
        let adj = &self.graph.adjacency;
        let row: &Vec<Option<i32>> = &adj[self.graph.width * node.1 + node.0];
        let mut nbs = vec![];

        for y in 0..self.graph.height {
            for x in 0..self.graph.width {
                match row[y * self.graph.width + x] {
                    None => {},
                    Some(_) => nbs.push((x, y))
                }
            }
        }
        
        nbs
    }
    
    fn get_steps(self: &mut Self, target_node: (usize, usize)) -> i32 {
        let mut current = target_node;
        let mut steps = 0;
        //println!("Pred {:?}", self.pred);
        while current != self.start_node {
            //println!("Current {:?}", current);
            steps += 1;
            current = self.pred[&current];
        }

        steps
    }
}
/*
impl BellmanFord {
    fn new(graph: Graph, start_node: (usize, usize)) -> Self {
        let mut distance = HashMap::new();
        for y in 0..graph.height {
            for x in 0..graph.width {
                distance.insert((x, y), 999999);
            }
        }
        distance.insert(start_node, 0);
        
        BellmanFord {
            graph,
            distance,
            pred: HashMap::new()
        }
    }
    
    fn relax(self: &mut Self) {
        for from_i in 0..self.graph.adjacency.len() {
            let from = (from_i % self.graph.width, from_i / self.graph.width);
            for to_i in 0..self.graph.adjacency.len() {
                if !self.graph.adjacency[from_i][to_i] { // No edge
                    continue;
                }

                

                // y * width + x
                let to = (to_i % self.graph.width, to_i / self.graph.width);
                //println!("Doing edge {:?}->{:?}", from, to);
                //println!("from {:?} to {:?}, gw {}, gh {}", from, to, self.graph.width, self.graph.height);
                if self.distance[&from] + 1 < self.distance[&to] {
                    self.distance.insert(to, self.distance[&from] + 1);
                    self.pred.insert(to, from);
                }                
            }
        }
    }
    
    fn find(self: &mut Self, start_node: (usize, usize), target_node: (usize, usize)) -> i32 {
        println!("Called find");
        let relaxations = (self.graph.width * self.graph.height) - 1;
        let pc1 = relaxations / 100;
        let elapsed = SystemTime::now();
        for step in 0..relaxations {
            //if step % pc1 == 0 {
                println!("{}/{} done", step, relaxations);
            //}
            self.relax();

            println!("{}/{} done", step, relaxations);
            let steps_remaining = (relaxations - step - 1) as f64;
            let time_per_step: f64 = elapsed.elapsed().unwrap().as_millis() as f64 / (step + 1) as f64;
            println!("Estimated time remaining: {} minutes", steps_remaining * time_per_step / 1000f64 / 60f64 / 60f64)
            
        }
        
        let mut current = target_node;
        let mut steps = 0;
        //println!("Pred {:?}", self.pred);
        while current != start_node {
            //println!("Current {:?}", current);
            steps += 1;
            current = self.pred[&current];
        }
        
        steps
    }
}

impl GraphSearch {
    fn new(graph: Graph, start_node: (usize, usize)) -> Self {
        let mut costs = HashMap::new();
        let mut visited = HashSet::new();
        costs.insert(start_node, 0);
        visited.insert(start_node);
        let mut gs = GraphSearch {
            graph,
            working: vec![],
            visited,
            costs
        };

        gs.visit_neighbours(start_node);
        
        gs
    }
    
    fn find_step(self: &mut GraphSearch, target_node: (usize, usize)) -> Option<&u32> {
        if self.visited.contains(&target_node) {
            return self.costs.get(&target_node)
        }

        let current_node = self.working.pop().unwrap();
        self.visit_neighbours(current_node);        
        
        None
    }
    
    fn visit_neighbours(self: &mut GraphSearch, current_node: (usize, usize)) {
        println!("visit_neighbours ( {:?} )", current_node);
        let neighbours = self.get_neighbours(current_node);

        for nb in neighbours {
            if !self.visited.contains(&nb) {
                self.visited.insert(nb);
                self.add_neighbours(nb);
            }
            
            let new_cost = self.costs.get(&current_node).unwrap() + 1;
            self.update_cost(nb, new_cost);
        }
    }
    
    fn update_cost(self: &mut GraphSearch, node: (usize, usize), new_cost: u32) {
        println!("update_cost ( {:?}, {} )", node, new_cost);
        match self.costs.get(&node) {
            None => {
                self.costs.insert(node, new_cost);
            }
            Some(&current_cost) => {
                self.costs.insert(node, min(current_cost, new_cost));
            }
        }
    }
    
    fn add_neighbours(self: &mut GraphSearch, node: (usize, usize)) {
        let neighbours = self.get_neighbours(node);
        
        for nb in neighbours {
            if !self.visited.contains(&nb) {
                self.working.push(nb);
            }
        }
    }
    
    fn get_neighbours(self: &mut GraphSearch, current_node: (usize, usize)) -> Vec<(usize, usize)> {
        let adj = &self.graph.adjacency;
        let row: &Vec<bool> = &adj[self.graph.width * current_node.1 + current_node.0];
        let mut nbs = vec![];
        
        for y in 0..self.graph.height {
            for x in 0..self.graph.width {
                if row[y * self.graph.width + x] {
                    nbs.push((x, y));
                }
            }
        }

        println!("get_neighbours( {:?} ) = {:?}", current_node, nbs);
        nbs
    }
}
*/
fn read_input() -> Input {
    let contents = match read_to_string("inputs/day12.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");
    
    let mut y = 0;
    let mut x = 0;
    let mut start = (0, 0);
    let mut end= (0, 0);
    
    let mut evs = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        x = 0;
        let row: Vec<u32> = line.chars().map(|c| {
            let ev = match c {
                'S' => {
                    start = (x, y);
                    0
                },
                'E' => {
                    end = (x, y);
                    25
                },
                k => k as u32 - 97
            };
            x += 1;
            ev
        }).collect();
        evs.push(row);
        y += 1;
    }
    
    Input {
        elevations: evs,
        start,
        end
    }
}

fn create_adjacency_matrix(elevations: &Vec<Vec<u32>>) -> Graph {
    let height = elevations.len();
    let width = elevations[0].len();
    let mut result = vec![];
    for y in 0..height {
        for x in 0..width {
            let mut row = vec![];
            for ny in 0..height {
                for nx in 0..width {
                    let t_dist = (x as i32 - nx as i32).abs() + (y as i32 - ny as i32).abs();
                    row.push(if t_dist == 1 {
                        let diff = elevations[ny][nx] as i32 - elevations[y][x] as i32;
                        if diff > 1 { None } else {Some(1)}
                        /*if diff > 1 { None } else { 
                            Some(match diff {
                                1 => 1,
                                0 => 1,
                                d => -d + 2
                            })
                        }*/
                    } else { None });
                }
            }
            result.push(row);
        }
        
    }
    
    Graph {
        width, height, adjacency: result
    }
}

pub fn solve() {
    let mut input = read_input();
    println!("Input read.");
    let graph = create_adjacency_matrix(&input.elevations);
    println!("Graph created");
    let part1 = solve_part1(&input, graph.clone());
    let part2 = solve_part2(&input, graph.clone());

    println!("Day 11");
    println!("Part 1: {}", part1); // 497
    println!("Part 2: {}", part2); // 492
}

fn solve_part1(input: &Input, graph: Graph) -> i32 {
    let mut gs = AStar::new(graph, input.start);
    gs.initialize(input.start);
    println!("A* initialized");
    
    gs.find(input.end).unwrap()
}

fn solve_part2(input: &Input, graph: Graph) -> i32 {
    let mut candidates = HashSet::new();
    for y in 0..graph.height {
        for x in 0..graph.width {
            if input.elevations[y][x] == 0 {
                let mut any_b = false;
                for (nx, ny) in [(x as i32 +1, y as i32), (x as i32-1,y as i32), (x as i32, y as i32+1), (x as i32,y as i32-1)] {
                    if (nx >= 0 && nx < graph.width as i32 && ny >= 0 && ny < graph.height as i32 && input.elevations[ny as usize][nx as usize] == 1) {
                        any_b = true;
                        break;
                    }
                }
                if any_b {
                    candidates.insert((x, y));
                }
            }
        }
    }
    
    println!("Candiates {:?}", candidates);
    let mut gs = AStar::new(graph, input.start);
    
    let mut result = vec![];
    for c in candidates {
        gs.initialize(c);
        let length = gs.find(input.end);
        if length.is_ok() {
            result.push(length.unwrap());
        }
    }
    
    println!("{:?}", result);
    
    *result.iter().min().unwrap()
}
