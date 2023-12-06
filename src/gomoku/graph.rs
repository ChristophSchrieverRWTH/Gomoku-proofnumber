use slotmap::{new_key_type, SlotMap};

new_key_type! {struct Key;}

pub struct PNS {
    tree: SlotMap<Key, Node>,
    root: Key,
}

#[derive(Debug, Clone)]
pub struct Node {
    depth: usize,
    proof: i32,
    disproof: i32,
    expanded: bool,
    state: Status,
    node_type: NodeType,
    parent: Key,
    children: Vec<Key>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Disproven,
    Proven,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    AND,
    OR,
}
