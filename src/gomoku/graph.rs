use slotmap::{new_key_type, SlotMap};
use std::collections::HashSet;

new_key_type! {pub struct Key;}
type Turn = (i32, i32);

#[derive(Debug, Clone)]
pub struct PNS {
    pub tree: SlotMap<Key, Node>,
    pub root: Key,
    pub legal: HashSet<Turn>,
}

#[derive(Debug, Clone)]
pub struct Node {
    turn: Option<Turn>,
    proof: i32,
    disproof: i32,
    expanded: bool,
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
    pub fn setup(size: i32) -> Self {
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
        }
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
}
