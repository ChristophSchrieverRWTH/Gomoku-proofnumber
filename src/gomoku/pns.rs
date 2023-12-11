use super::game::*;
use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{self, Hash, Hasher};
use std::{thread, time};

use std::time::Instant;

type Turn = (i32, i32);

#[derive(Debug)]
pub struct PNS {
    pub tree: HashMap<u64, Node>,
    pub root: u64,
    pub legal: HashSet<Turn>,
    pub board: Board,
    pub draw_is_loss: bool,
}

#[derive(Debug, Clone)]
pub struct Node {
    turn: Option<Turn>,
    proof: i32,
    disproof: i32,
    pub expanded: bool,
    state: Status,
    node_type: NodeType,
    parent: Option<u64>,
    children: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Disproven,
    Proven,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    AND,
    OR,
}

impl PNS {
    pub fn setup(
        size: i32,
        shape1: &mut Vec<(i32, i32)>,
        shape2: &mut Vec<(i32, i32)>,
        draw_is_loss: bool,
        moves_made: Vec<Turn>,
    ) -> Self {
        let mut legal: HashSet<Turn> = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                legal.insert((i, j));
            }
        }
        let mut tree = HashMap::new();
        let mut board = Board::setup(size, shape1, shape2);
        let mut root_type = match moves_made.len() % 2 {
            0 => NodeType::OR,
            _ => NodeType::AND,
        };
        for (x_cord, y_cord) in moves_made {
            board.place_proof(x_cord, y_cord);
            legal.remove(&(x_cord, y_cord));
        }
        let root = calculate_hash(&board.field);
        let root_node = Node {
            turn: None,
            proof: 1,
            disproof: 1,
            expanded: false,
            state: Status::Unknown,
            node_type: root_type,
            parent: None,
            children: vec![],
        };
        tree.insert(root, root_node);
        PNS {
            tree,
            root,
            legal,
            board,
            draw_is_loss,
        }
    }

    pub fn generate_children(&mut self, hash: u64) {
        let parent = self.tree.get(&hash).unwrap();
        let node_type = match parent.node_type {
            NodeType::AND => NodeType::OR,
            NodeType::OR => NodeType::AND,
        };
        let mut child_hashes = vec![];
        for (i, j) in &self.legal {
            self.board.place_proof(*i, *j);
            let child_hash = calculate_hash(&self.board.field);
            if self.tree.get(&child_hash).is_none() {
                let node: Node = Node {
                    turn: Some((*i, *j)),
                    proof: 1,
                    disproof: 1,
                    expanded: false,
                    state: Status::Unknown,
                    node_type,
                    parent: Some(hash),
                    children: vec![],
                };
                self.tree.insert(child_hash, node);
                child_hashes.push(child_hash);
            } else {
                // println!("It happened!");
            }
            //child_hashes.push(child_hash);
            self.board.undo(*i, *j);
        }
        let parent = self.tree.get_mut(&hash).unwrap();
        parent.children = child_hashes;
    }

    pub fn evaluate(&mut self, hash: u64) {
        let mut state;
        if hash == self.root || !self.board.is_over() {
            state = Status::Unknown;
        } else {
            state = match self.board.winner {
                Tile::One => Status::Proven,
                Tile::Two => Status::Disproven,
                Tile::Empty => match self.draw_is_loss {
                    true => Status::Disproven,
                    false => Status::Proven,
                },
            };
        }
        self.tree.get_mut(&hash).unwrap().state = state;
    }

    pub fn expand(&mut self, hash: u64) {
        let node = self.tree.get(&hash).unwrap();
        if node.expanded {
            return;
        }
        self.generate_children(hash);
        let node = self.tree.get(&hash).unwrap();
        let n_type = node.node_type.clone();
        let children = node.children.clone();
        for child_hash in children {
            let child = self.tree.get(&child_hash).unwrap();
            let turn = child.turn.unwrap();
            self.board.place_proof(turn.0, turn.1);
            self.evaluate(child_hash);
            self.set_numbers(child_hash);
            let child = self.tree.get(&child_hash).unwrap();
            let turn = child.turn.unwrap();
            self.board.undo(turn.0, turn.1);
            if (n_type == NodeType::OR && child.proof == 0)
                || (n_type == NodeType::AND && child.disproof == 0)
            {
                break;
            }
        }
        let node = self.tree.get_mut(&hash).unwrap();
        node.expanded = true;
    }

    pub fn set_numbers(&mut self, hash: u64) {
        let mut node = self.tree.get(&hash).unwrap();
        if node.expanded {
            let node = self.tree.get(&hash).unwrap();
            let mut temp_proof: i32;
            let mut temp_disproof: i32;
            match node.node_type {
                NodeType::AND => {
                    temp_proof = 0;
                    temp_disproof = f32::INFINITY as i32;
                    for child_hash in &node.children {
                        let child = self.tree.get(child_hash).unwrap();
                        temp_proof = temp_proof.saturating_add(child.proof);
                        temp_disproof = min(child.disproof, temp_disproof);
                    }
                }
                NodeType::OR => {
                    temp_proof = f32::INFINITY as i32;
                    temp_disproof = 0;
                    for child_hash in &node.children {
                        let child = self.tree.get(child_hash).unwrap();
                        temp_disproof = temp_disproof.saturating_add(child.disproof);
                        temp_proof = min(child.proof, temp_proof);
                    }
                }
            }
            let node = self.tree.get_mut(&hash).unwrap();
            node.proof = temp_proof;
            node.disproof = temp_disproof;
        } else {
            let mut node = self.tree.get_mut(&hash).unwrap();
            (node.proof, node.disproof) = match node.state {
                Status::Disproven => (f32::INFINITY as i32, 0),
                Status::Proven => (0, f32::INFINITY as i32),
                Status::Unknown => (1, 1),
            }
        }
    }

    pub fn select_mpn(&mut self, hash: u64) -> u64 {
        let mut best = hash;
        let mut answer_hash = hash;
        loop {
            let ten_millis = time::Duration::from_millis(300);
            let now = time::Instant::now();
            //thread::sleep(ten_millis);
            let mut value = f32::INFINITY as i32;
            let node = self.tree.get(&answer_hash).unwrap();
            // println!("\n{:?}\n", node);
            // println!("board belongs to above node: \n{:?}", self.board);
            let n_type = node.node_type.clone();
            if !node.expanded {
                break;
            }
            match n_type {
                NodeType::OR => {
                    for child_hash in &node.children {
                        let child = self.tree.get(child_hash).unwrap();
                        if value > child.proof {
                            best = *child_hash;
                            value = child.proof;
                        }
                    }
                }
                NodeType::AND => {
                    for child_hash in &node.children {
                        let child = self.tree.get(child_hash).unwrap();
                        if value > child.disproof {
                            best = *child_hash;
                            value = child.disproof;
                        }
                    }
                }
            }
            let turn = self.tree.get(&best).unwrap().turn.unwrap();
            self.board.place_proof(turn.0, turn.1);
            self.legal.remove(&turn);
            answer_hash = best;
        }
        answer_hash
    }

    pub fn update_ancestors(&mut self, hash: u64, root_hash: u64) -> u64 {
        let mut node_hash = hash;
        loop {
            let node = self.tree.get(&node_hash).unwrap();
            let old_proof = node.proof;
            let old_disproof = node.disproof;
            self.set_numbers(node_hash);
            let node = self.tree.get_mut(&node_hash).unwrap();
            if node.proof == old_proof {
                node.disproof = old_disproof;
                return node_hash;
            }
            if node_hash == root_hash {
                return node_hash;
            }
            node_hash = node.parent.unwrap();
            let turn = node.turn.unwrap();
            self.board.undo(turn.0, turn.1);
            self.legal.insert(turn);
        }
    }

    pub fn pns(&mut self, root_hash: u64) -> (i32, i32) {
        self.evaluate(root_hash);
        self.set_numbers(root_hash);
        let mut current = root_hash.clone();
        let mut most_proving: u64;
        loop {
            let root = self.tree.get(&root_hash).unwrap();
            println!(
                "Root proofnumber: {}, Root disproofnumber: {}",
                root.proof, root.disproof
            );
            if root.proof == 0 || root.disproof == 0 {
                break;
            }
            most_proving = self.select_mpn(current);
            self.expand(most_proving);
            current = self.update_ancestors(most_proving, root_hash);
        }
        let root = self.tree.get(&root_hash).unwrap();
        (root.proof, root.disproof)
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
