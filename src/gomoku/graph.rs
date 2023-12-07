#![allow(unused)]
use super::game::*;
use slotmap::{new_key_type, SlotMap};
use std::cmp::min;
use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq)]
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
    ) -> Self {
        let mut hs = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                hs.insert((i, j));
            }
        }
        let mut sm: SlotMap<Key, Node> = SlotMap::with_key();
        let root = Node {
            turn: None,
            proof: 1,
            disproof: 1,
            expanded: false,
            state: Status::Unknown,
            node_type: NodeType::OR,
            parent: None,
            children: vec![],
        };
        let key = sm.insert(root);
        PNS {
            tree: sm,
            root: key,
            legal: hs,
            board: Board::setup(size, shape1, shape2),
            draw_is_loss,
        }
    }

    pub fn pns(&mut self, root_key: Key) -> Status {
        self.evaluate(root_key);
        self.set_numbers(root_key);
        Status::Proven
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
                        temp_proof = temp_proof + child.proof;
                        temp_disproof = min(child.disproof, temp_disproof);
                    }
                }
                NodeType::OR => {
                    temp_proof = f32::INFINITY as i32;
                    temp_disproof = 0;
                    for child_key in &node.children {
                        let child = self.tree.get(*child_key).unwrap();
                        temp_disproof = temp_disproof + child.disproof;
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
