#![allow(unused)]
use super::game::*;
use slotmap::{new_key_type, SlotMap};
use std::cmp::min;
use std::collections::HashSet;
use std::{thread, time};

new_key_type! {pub struct Key;}
type Turn = (i32, i32);

#[derive(Debug)]
pub struct PNS {
    pub tree: SlotMap<Key, Node>,
    pub root: Key,
    pub legal: HashSet<Turn>,
    pub board: Board,
    pub draw_is_loss: bool,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub turn: Option<Turn>,
    proof: i32,
    disproof: i32,
    pub expanded: bool,
    state: Status,
    node_type: NodeType,
    parent: Option<Key>,
    children: Vec<Key>,
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
        let mut hs = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                hs.insert((i, j));
            }
        }
        let mut sm: SlotMap<Key, Node> = SlotMap::with_key();
        let mut board = Board::setup(size, shape1, shape2);
        let mut root_type = match moves_made.len() % 2 {
            0 => NodeType::OR,
            _ => NodeType::AND,
        };
        for (x_cord, y_cord) in moves_made {
            board.place_proof(x_cord, y_cord);
            hs.remove(&(x_cord, y_cord));
        }
        let root = Node {
            turn: None,
            proof: 1,
            disproof: 1,
            expanded: false,
            state: Status::Unknown,
            node_type: root_type,
            parent: None,
            children: vec![],
        };
        let key = sm.insert(root);
        PNS {
            tree: sm,
            root: key,
            legal: hs,
            board: board,
            draw_is_loss,
        }
    }

    pub fn pns(&mut self, root_key: Key) -> (i32, i32) {
        self.evaluate(root_key);
        self.set_numbers(root_key);
        let mut current = root_key.clone();
        let mut most_proving: Key;
        loop {
            let root = self.tree.get(root_key).unwrap();
            println!(
                "Root proofnumber: {}, Root disproofnumber: {}",
                root.proof, root.disproof
            );
            if root.proof == 0 || root.disproof == 0 {
                break;
            }
            most_proving = self.select_mpn(current);
            self.expand(most_proving);
            current = self.update_ancestors(most_proving, root_key);
        }
        let root = self.tree.get(root_key).unwrap();
        (root.proof, root.disproof)
    }

    pub fn update_ancestors(&mut self, key: Key, root_key: Key) -> Key {
        let mut node_key = key;
        loop {
            let node = self.tree.get(node_key).unwrap();
            let old_proof = node.proof.clone(); // maybe clone
            let old_disproof = node.disproof.clone();
            self.set_numbers(node_key);
            let node = self.tree.get_mut(node_key).unwrap();
            if node.proof == old_proof {
                node.disproof = old_disproof;
                return node_key;
            }
            if node_key == root_key {
                return node_key;
            }
            node_key = node.parent.unwrap();
            let turn = node.turn.unwrap();
            self.board.undo(turn.0, turn.1);
            self.legal.insert(turn);
        }
    }

    pub fn select_mpn(&mut self, key: Key) -> Key {
        let ten_millis = time::Duration::from_millis(300);
        let now = time::Instant::now();
        let mut best = key;
        let mut answer_key = key;
        loop {
            let mut value = f32::INFINITY as i32;
            let node = self.tree.get(answer_key).unwrap();
            let n_type = node.node_type.clone();
            if !node.expanded {
                break;
            }
            match n_type {
                NodeType::OR => {
                    for child_key in &node.children {
                        let child = self.tree.get(*child_key).unwrap();
                        if value > child.proof {
                            best = *child_key;
                            value = child.proof;
                        }
                    }
                }
                NodeType::AND => {
                    for child_key in &node.children {
                        let child = self.tree.get(*child_key).unwrap();
                        if value > child.disproof {
                            best = *child_key;
                            value = child.disproof;
                        }
                    }
                }
            }
            let turn = self.tree.get(best).unwrap().turn.unwrap();
            self.board.place_proof(turn.0, turn.1);
            self.legal.remove(&turn);
            answer_key = best;
        }
        answer_key
    }

    pub fn expand(&mut self, key: Key) {
        self.generate_children(key);
        let node = self.tree.get(key).unwrap();
        let n_type = node.node_type.clone();
        let children = node.children.clone();
        for child_key in children {
            let child = self.tree.get(child_key).unwrap();
            let turn = child.turn.expect("Function should not be called on root");
            self.board.place_proof(turn.0, turn.1);
            self.evaluate(child_key);
            self.set_numbers(child_key);
            let child = self.tree.get(child_key).unwrap();
            self.board.undo(turn.0, turn.1);
            if (n_type == NodeType::OR && child.proof == 0)
                || (n_type == NodeType::AND && child.disproof == 0)
            {
                break;
            }
        }
        let node = self.tree.get_mut(key).unwrap();
        node.expanded = true;
    }

    pub fn generate_children(&mut self, key: Key) {
        let parent = self.tree.get(key).unwrap();
        let node_type = match parent.node_type {
            NodeType::AND => NodeType::OR,
            NodeType::OR => NodeType::AND,
        };
        let mut child_keys = vec![];
        for (i, j) in &self.legal {
            let child: Node = Node {
                turn: Some((*i, *j)),
                proof: 1,
                disproof: 1,
                expanded: false,
                state: Status::Unknown,
                node_type,
                parent: Some(key),
                children: vec![],
            };
            let child_key = self.tree.insert(child);
            child_keys.push(child_key);
        }
        let parent = self.tree.get_mut(key).unwrap();
        parent.children = child_keys;
    }

    pub fn evaluate(&mut self, key: Key) {
        let mut state;
        if key == self.root || !self.board.is_over() {
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
        self.tree.get_mut(key).unwrap().state = state;
    }

    pub fn set_numbers(&mut self, key: Key) {
        let mut node = self.tree.get(key).unwrap();
        if node.expanded {
            let node = self.tree.get(key).unwrap();
            let mut temp_proof: i32;
            let mut temp_disproof: i32;
            match node.node_type {
                NodeType::AND => {
                    temp_proof = 0;
                    temp_disproof = f32::INFINITY as i32;
                    for child_key in &node.children {
                        let child = self.tree.get(*child_key).unwrap();
                        temp_proof = temp_proof.saturating_add(child.proof);
                        temp_disproof = min(child.disproof, temp_disproof);
                    }
                }
                NodeType::OR => {
                    temp_proof = f32::INFINITY as i32;
                    temp_disproof = 0;
                    for child_key in &node.children {
                        let child = self.tree.get(*child_key).unwrap();
                        temp_disproof = temp_disproof.saturating_add(child.disproof);
                        temp_proof = min(child.proof, temp_proof);
                    }
                }
            }
            let node = self.tree.get_mut(key).unwrap();
            node.proof = temp_proof;
            node.disproof = temp_disproof;
        } else {
            let mut node = self.tree.get_mut(key).unwrap();
            (node.proof, node.disproof) = match node.state {
                Status::Disproven => (f32::INFINITY as i32, 0),
                Status::Proven => (0, f32::INFINITY as i32),
                Status::Unknown => (1, 1),
            }
        }
    }
}
