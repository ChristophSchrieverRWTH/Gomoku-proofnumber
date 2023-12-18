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
    phi: i32,
    delta: i32,
    state: Status,
    parent: Option<Key>,
    children: Vec<Key>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Disproven,
    Proven,
    Unknown,
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
        for (x_cord, y_cord) in moves_made {
            board.place_proof(x_cord, y_cord);
            hs.remove(&(x_cord, y_cord));
        }
        let root = Node {
            turn: None,
            phi: 1,
            delta: 1,
            state: Status::Unknown,
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

    pub fn dfpn(&mut self, root_key: Key) -> Status {
        let root = self.tree.get_mut(root_key).unwrap();
        root.phi = f32::INFINITY as i32;
        root.delta = f32::INFINITY as i32;
        self.mid(root_key);
        let root = self.tree.get_mut(root_key).unwrap();
        if root.delta != 0 {
            Status::Proven
        } else {
            Status::Disproven
        }
    }

    pub fn mid(&mut self, key: Key) {
        let node = self.tree.get(key).unwrap();
        let turn = node.turn.unwrap();
        self.board.place_proof(turn.0, turn.1);
        if self.board.is_over() {
            self.evaluate(key);
            self.board.undo(turn.0, turn.1);
        } else {
            self.legal.remove(&turn);
            self.generate_children(key);
            loop {
                let node = self.tree.get(key).unwrap();
                let phi_sum = self.sum(key);
                let n_phi = node.phi;
                let n_delta = node.delta;
                if !(node.phi > self.min(key) && node.delta > self.sum(key)) {
                    break;
                }
                let (best_key, phi_c, delta2) = self.select_child(key);
                let best_child = self.tree.get_mut(best_key).unwrap();
                let best_turn = best_child.turn.unwrap();
                self.board.place_proof(best_turn.0, best_turn.1);
                best_child.phi = n_delta + phi_c - phi_sum;
                best_child.delta = std::cmp::min(n_phi, delta2 + 1);
                self.mid(best_key);
            }
            let delta_min = self.min(key);
            let phi_sum = self.sum(key);
            let node = self.tree.get_mut(key).unwrap();
            node.phi = delta_min;
            node.delta = phi_sum;
        }
    }

    pub fn select_child(&mut self, key: Key) -> (Key, i32, i32) {
        let mut delta_c = f32::INFINITY as i32;
        let mut phi_c = f32::INFINITY as i32;
        let mut delta2 = f32::INFINITY as i32;
        let node = self.tree.get(key).unwrap();
        let mut best_key = key;
        for child_key in &node.children {
            let child = self.tree.get(*child_key).unwrap();
            let delta = child.delta;
            let phi = child.phi;
            if delta < delta_c {
                best_key = *child_key;
                delta2 = delta_c;
                phi_c = phi;
                delta_c = delta;
            } else if delta < delta2 {
                delta2 = delta;
            }
            if phi != 0 {
                return (best_key, phi_c, delta2);
            }
        }
        (best_key, phi_c, delta2)
    }

    pub fn evaluate(&mut self, key: Key) {
        let state = match self.board.winner {
            Tile::One => Status::Proven,
            Tile::Two => Status::Disproven,
            Tile::Empty => match self.draw_is_loss {
                true => Status::Disproven,
                false => Status::Proven,
            },
        };

        self.tree.get_mut(key).unwrap().state = state;
    }

    pub fn generate_children(&mut self, key: Key) {
        let parent = self.tree.get(key).unwrap();
        let mut child_keys = vec![];
        for (i, j) in &self.legal {
            let child: Node = Node {
                turn: Some((*i, *j)),
                phi: 1,
                delta: 1,
                state: Status::Unknown,
                parent: Some(key),
                children: vec![],
            };
            let child_key = self.tree.insert(child);
            child_keys.push(child_key);
        }
        let parent = self.tree.get_mut(key).unwrap();
        parent.children = child_keys;
    }

    pub fn sum(&self, key: Key) -> i32 {
        let mut sum = 0;
        let node = self.tree.get(key).unwrap();
        for child_key in &node.children {
            let child = self.tree.get(*child_key).unwrap();
            sum += child.phi;
        }
        sum
    }

    pub fn min(&self, key: Key) -> i32 {
        let mut min = f32::INFINITY as i32;
        let node = self.tree.get(key).unwrap();
        for child_key in &node.children {
            let child = self.tree.get(*child_key).unwrap();
            min = std::cmp::min(min, child.delta);
        }
        min
    }
}
