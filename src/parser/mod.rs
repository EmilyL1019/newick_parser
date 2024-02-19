use std::{cmp, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
struct Leaf<T> {
    value: T,
    distance: Option<f32>,
    index: i32,
}

#[derive(Debug, Clone)]
pub struct Tree<T> {
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    left_leaf: Option<Leaf<T>>,
    right_leaf: Option<Leaf<T>>,
    probability: Option<f32>,
    index: i32,
    id: String,
}

impl<'a, T: Clone> Tree<T> {
    pub fn new() -> Self {
        Tree {
            left: None,
            right: None,
            left_leaf: None,
            right_leaf: None,
            probability: None,
            index: 0,
            id: "".to_string(),
        }
    }
}

pub fn get_small_tree_string(newick: &String) -> String {
    let mut tree_string: String = "".to_string();
    let mut left_parenthesis_index = 0;
    let mut right_parenthesis_index = 0;
    let mut found_colon_after_tree = false;
    for (i, c) in newick.chars().enumerate() {
        if !found_colon_after_tree {
            match c {
                '(' => {
                    left_parenthesis_index = i;
                    tree_string = c.to_string();
                }
                ')' => {
                    right_parenthesis_index = i;
                    tree_string = tree_string + &c.to_string();
                }
                ':' => {
                    if right_parenthesis_index > left_parenthesis_index {
                        found_colon_after_tree = true;
                    } else {
                        tree_string = tree_string + &c.to_string();
                    }
                }
                //Special condition required if only one comma is left in the newick
                ',' => {
                    if right_parenthesis_index < left_parenthesis_index
                        || (left_parenthesis_index == 0 && right_parenthesis_index == 0)
                    {
                        tree_string = tree_string + &c.to_string();
                    }
                }
                _ => tree_string = tree_string + &c.to_string(),
            }
        }
    }
    tree_string
}

fn is_tree(branch: Vec<char>) -> bool {
    let b: String = branch.iter().collect();
    let mut index = 2;
    if branch[0] == 'T' && branch[1] == '-' {
        while index < branch.len() - 1 && (branch[index].is_digit(10) || branch[index] == '.') {
            index = index + 1;
            if branch[index] == ':' {
                index = index + 1;
            }
        }
    }
    //Need to add 1 because index starts at 0 and length starts at 1
    index = index + 1;
    index == branch.len()
}

// Builds a single leaf
fn create_leaf(input_leaf: &String) -> Leaf<String> {
    let leaf = String::from(input_leaf);
    let parts: Vec<String> = leaf.split(":").map(str::to_string).collect();
    let leaf_split: Vec<String> = parts[0].split("-").map(str::to_string).collect();
    let leaf: Leaf<String> = match parts.len() {
        1 => match leaf_split.len() {
            1 => Leaf {
                value: parts[0].clone(),
                distance: None,
                index: 0,
            },
            2 => Leaf {
                value: leaf_split[0].clone(),
                distance: None,
                index: 0,
            },
            _ => {
                eprintln!("Incorrect Newick Format");
                std::process::exit(0);
            }
        },
        2 => match leaf_split.len() {
            1 => Leaf {
                value: parts[0].clone(),
                distance: Some(parts[1].parse::<f32>().unwrap()),
                index: 0,
            },
            2 => Leaf {
                value: leaf_split[0].clone(),
                distance: Some(parts[1].parse::<f32>().unwrap()),
                index: 0,
            },
            _ => {
                eprintln!("Incorrect Newick Format");
                std::process::exit(0);
            }
        },
        _ => {
            eprintln!("Incorrect Newick Format");
            std::process::exit(0);
        }
    };
    leaf
}

// Decides which type of tree a string is
fn type_tree(branches: &Vec<String>) -> u8 {
    let l_tree = is_tree(branches[0].chars().collect());
    let r_tree = is_tree(branches[1].chars().collect());
    match l_tree {
        true => match r_tree {
            true => 0,
            false => 1,
        },
        false => match r_tree {
            true => 2,
            false => 3,
        },
    }
}

// Build smallest tree out of the given tree
fn get_small_tree(
    newick: &String,
    trees: &[Tree<String>],
    tree_ids: Vec<String>,
) -> (
    String,
    String,
    Option<usize>,
    Option<usize>,
    Vec<String>,
    Option<f32>,
) {
    println!("newick start {}", newick);
    let tree_string: String = get_small_tree_string(&newick);
    println!("tree string: {}", tree_string);
    let divide: Vec<&str> = tree_string.split(')').collect();
    // First get branches
    let branches: Vec<String> = divide[0][1..].split(',').map(str::to_string).collect();
    let mut branch_names: Vec<String> = Vec::new();
    for branch in &branches {
        branch_names.push(
            branch
                .split(':')
                .map(str::to_string)
                .collect::<Vec<String>>()[0]
                .clone(),
        );
    }
    let t_probability: Option<f32> = {
        if divide[1].len() > 0 {
            Some(divide[1].parse::<f32>().unwrap())
        } else {
            None
        }
    };

    let l_tree_index = tree_ids.iter().position(|r| *r == branch_names[0]);
    let r_tree_index = tree_ids.iter().position(|r| *r == branch_names[1]);
    let tree_id: String = "T-".to_string() + &trees.len().to_string();
    (
        tree_string,
        tree_id,
        l_tree_index,
        r_tree_index,
        branches,
        t_probability,
    )
}

fn build_tree<'a>(mut newick: String) -> Vec<Tree<String>> {
    let mut trees: Vec<Tree<String>> = Vec::new();
    let mut tree_ids: Vec<String> = Vec::new();
    while newick.contains(",") {
        let (tree_string, tree_id, l_tree_index, r_tree_index, branches, t_probability) =
            get_small_tree(&newick, &trees, tree_ids.clone());
        newick = newick.replace(&tree_string, tree_id.as_str());
        println!("Tree num: {}", type_tree(&branches));
        let tree = match type_tree(&branches) {
            0 => Tree {
                left: Some(Box::new(trees[l_tree_index.unwrap()].clone())),
                right: Some(Box::new(trees[r_tree_index.unwrap()].clone())),
                left_leaf: None,
                right_leaf: None,
                probability: t_probability,
                index: cmp::max(
                    trees[l_tree_index.unwrap()].index,
                    trees[r_tree_index.unwrap()].index,
                ),
                id: tree_id.clone(),
            },
            1 => Tree {
                left: Some(Box::new(trees[l_tree_index.unwrap()].clone())),
                right: None,
                left_leaf: None,
                right_leaf: Some(create_leaf(&branches[1])),
                probability: t_probability,
                index: trees[l_tree_index.unwrap()].index,
                id: tree_id.clone(),
            },
            2 => Tree {
                left: None,
                right: Some(Box::new(trees[r_tree_index.unwrap()].clone())),
                left_leaf: Some(create_leaf(&branches[0])),
                right_leaf: None,
                probability: t_probability,
                index: trees[r_tree_index.unwrap()].index,
                id: tree_id.clone(),
            },
            _ => Tree {
                left: None,
                right: None,
                left_leaf: Some(create_leaf(&branches[0])),
                right_leaf: Some(create_leaf(&branches[1])),
                probability: t_probability,
                index: 1,
                id: tree_id.clone(),
            },
        };
        trees.push(tree);
        tree_ids.push(tree_id);
    }
    trees
}

pub fn create_final_tree(newick: String) -> Tree<String> {
    let trees = build_tree(newick);
    trees[trees.len() - 1].clone()
}

pub fn print_tree<T: Display>(tree: &Tree<T>) {
    match tree.left_leaf.as_ref() {
        Some(x) => {
            print!("Left {} ", x.value);
        }
        None => {
            let left_tree = tree.left.as_ref().unwrap();
            println!("T Left");
            print_tree(left_tree);
        }
    }
    match tree.right_leaf {
        Some(ref x) => {
            println!("Right {} ", x.value);
        }
        None => {
            let leaf = tree.right.as_ref().unwrap();
            println!("T Right {} I {}", &leaf.id, leaf.index);
        }
    }
}

pub fn divide_tree(tree: Tree<String>) -> Vec<Tree<String>> {
    match tree.left {
        Some(_) => match tree.right {
            Some(_) => vec![*tree.left.unwrap(), *tree.right.unwrap()],
            None => vec![
                *tree.left.unwrap(),
                Tree {
                    left: None,
                    right: None,
                    left_leaf: None,
                    right_leaf: tree.right_leaf,
                    probability: None,
                    index: 0,
                    id: "".to_string(),
                },
            ],
        },
        None => match tree.right {
            Some(_) => vec![
                Tree {
                    left: None,
                    right: None,
                    left_leaf: tree.left_leaf,
                    right_leaf: None,
                    probability: None,
                    index: 0,
                    id: "".to_string(),
                },
                *tree.right.unwrap(),
            ],
            None => vec![
                Tree {
                    left: None,
                    right: None,
                    left_leaf: tree.left_leaf,
                    right_leaf: None,
                    probability: None,
                    index: 0,
                    id: "".to_string(),
                },
                Tree {
                    left: None,
                    right: None,
                    left_leaf: None,
                    right_leaf: tree.right_leaf,
                    probability: None,
                    index: 0,
                    id: "".to_string(),
                },
            ],
        },
    }
}
