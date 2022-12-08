use std::cell::RefCell;
use std::rc::Rc;
use crate::utility::utils::parse_file;

#[derive(Debug)]
enum Command {
    ChangeDir(String),
    IsFile(i32),
    IsDirectory(String),
    NoOp
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FileSystemItem {
    File(i32),
    Directory(String),
    Root // Special directory that is its own parent
}

#[derive(Debug)]
#[derive(PartialEq)]
struct TreeNode {
    label: FileSystemItem,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(label: FileSystemItem) -> TreeNode {
        return TreeNode {
            label,
            children: vec![],
            parent: None
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
        self.children.push(child);
    }
}

fn read_input() -> Vec<Command> {
    parse_file("inputs/day07.txt", |line| {
        if line.starts_with("$ cd") {
            let parts = line.split_at(5);
            Command::ChangeDir(parts.1.to_string())
        } else if line.starts_with("dir") {
            let parts = line.split_at(4);
            Command::IsDirectory(parts.1.to_string())
        } else if line.starts_with("$ ls") {
            Command::NoOp
        } else if !line.is_empty() {
            let parts = line.split_once(" ");
            match parts {
                None => panic!("Malformed line: {}", line),
                Some((size, _)) => {
                    let size_num: i32 = size.parse().unwrap();
                    Command::IsFile(size_num)
                }
            }
        } else {
            Command::NoOp
        }
    })
}

fn build_fsystem(cmds: &Vec<Command>) -> Rc<RefCell<TreeNode>> {
    let mut current = Rc::new(RefCell::new(TreeNode::new(FileSystemItem::Root)));
    //current.borrow_mut().label = Some(FileSystemItem::Root);

    for cmd in cmds {
        match cmd {
            Command::ChangeDir(dir) => {
                if dir == "/" {
                    // do nothing rn
                } else if dir == ".." {
                    let new_rc = match &current.borrow().parent {
                        None => Rc::clone(&current), // Do nothing, it is probably the root node
                        Some(parent) => {
                            Rc::clone(parent)
                        }
                    };
                    current = new_rc;
                } else {
                    current = {
                        let current_borrow = current.borrow();
                        let child = current_borrow.children.iter().find(|child| {
                            match &child.borrow().label {
                                FileSystemItem::File(_) => false,
                                FileSystemItem::Directory(name) => name == dir,
                                FileSystemItem::Root => dir == "/"
                            }
                        });
                        match child {
                            None => Rc::clone(&current),
                            Some(child_dir) => Rc::clone(child_dir)
                        }
                    }
                }
            }
            Command::IsFile(size) => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(FileSystemItem::File(*size))));
                new_node.borrow_mut().parent = Some(Rc::clone(&current));
                let _ = &current.borrow_mut().add_child(new_node);
            }
            Command::IsDirectory(name) => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(FileSystemItem::Directory(name.to_string()))));
                new_node.borrow_mut().parent = Some(Rc::clone(&current));
                let _ = &current.borrow_mut().add_child(new_node);
            }
            Command::NoOp => {
                continue;
            }
        }
    }
    
    loop {
        let new_rc = {
            let current_borrow = current.borrow_mut();
            if current_borrow.label == FileSystemItem::Root {
                break;
            }

            let parent = &current_borrow.parent.as_ref().unwrap();
            Rc::clone(parent)
        };
        
        current = new_rc;
    }
    
    current
}

fn get_size(tree: &Rc<RefCell<TreeNode>>, mut sizes: &mut Vec<i32>) -> i32 {
    let b = tree.borrow_mut();
    match b.label {
        FileSystemItem::File(size) => size,
        _ => {
            let size = b.children.iter().map(|c| get_size(c, &mut sizes)).sum();
            sizes.push(size);
            size
        }
    }
}

pub fn solve() {
    let input = read_input();
    let fsystem = build_fsystem(&input);

    let part1 = solve_part1(&fsystem);
    let part2 = solve_part2(&fsystem);
    
    println!("Day 06");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(fsys: &Rc<RefCell<TreeNode>>) -> i32 {
    let mut sizes = Vec::new();
    get_size(fsys, &mut sizes);
    sizes.into_iter().filter(|size| *size <= 100000).sum()
}

fn solve_part2(fsys: &Rc<RefCell<TreeNode>>) -> i32 {
    let mut sizes = Vec::new();
    let unused = 70_000_000 - get_size(fsys, &mut sizes);
    sizes.sort();
    sizes.into_iter().find(|size| unused + size >= 30_000_000).unwrap()
}
