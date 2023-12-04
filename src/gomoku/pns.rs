#![allow(unused)]
use std::rc::Rc;
type Turn = (i32, i32);

struct PNS {
    path: Vec<Turn>,
}

impl PNS {
    fn get_parent(&mut self) -> &mut Node {
        todo!()
    }

    fn get_node(&self) -> &mut Node {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Node {
    depth: usize,
    proof: i32,
    disproof: i32,
    expanded: bool,
    state: Status,
    node_type: NodeType,
    parent: Rc<Node>,
    children: Vec<Node>,
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

impl Node {
    fn new(parent: Rc<Node>) -> Self {
        Node {
            depth: 0,
            proof: 1,
            disproof: 1,
            expanded: false,
            state: Status::Unknown,
            parent,
            node_type: NodeType::AND,
            children: vec![],
        }
    }
}

fn pns(mut root: Node) {
    evaluate(&mut root);
    set_numbers(&mut root);
    let mut current: &mut Node = &mut root.clone();
    let mut most_proving: &mut Node;
    while root.proof != 0 && root.disproof != 0 {
        most_proving = select_most_proving(current);
        expand(most_proving);
        current = update_ancestors(most_proving, &root);
    }
}

fn evaluate(node: &mut Node) {}

fn set_numbers(node: &mut Node) {
    if node.expanded {
        if node.node_type == NodeType::AND {
            node.proof = 0;
            node.disproof = f32::INFINITY as i32;
            for child in &node.children {
                node.proof = node.proof + child.proof;
                if child.disproof < node.disproof {
                    node.disproof = child.disproof;
                }
            }
        } else {
            node.proof = f32::INFINITY as i32;
            node.disproof = 0;
            for child in &node.children {
                node.disproof = node.disproof + child.disproof;
                if child.proof < node.proof {
                    node.proof = child.proof;
                }
            }
        }
    } else {
        match node.state {
            Status::Disproven => {
                node.proof = f32::INFINITY as i32;
                node.disproof = 0;
            }
            Status::Proven => {
                node.proof = 0;
                node.disproof = f32::INFINITY as i32;
            }
            Status::Unknown => {
                node.proof = 1;
                node.disproof = 1;
            }
        }
    }
}

fn select_most_proving(node: &mut Node) -> &mut Node {
    let mut current = node;
    let mut value = f32::INFINITY as i32;
    let mut best = None;
    let mut expanded = true;
    while expanded {
        if current.node_type == NodeType::OR {
            for child in &mut current.children[..] {
                if value > child.proof {
                    value = child.proof;
                    best = Some(child);
                }
            }
        } else {
            for child in &mut current.children[..] {
                if value > child.disproof {
                    value = child.disproof;
                    best = Some(child);
                }
            }
        }
        current = best.take().unwrap();
        expanded = current.expanded;
    }
    current
}

fn expand(node: &mut Node) {
    generate_children(node);
    for child in &mut node.children {
        evaluate(child);
        set_numbers(child);
        if (node.node_type == NodeType::OR && child.proof == 0)
            || (node.node_type == NodeType::AND && child.disproof == 0)
        {
            break;
        }
    }
    node.expanded = true;
}

fn update_ancestors<'a>(node: &'a mut Node, root: &Node) -> &'a mut Node {
    let node = node;
    loop {
        let oldProof = node.proof;
        let oldDisproof = node.disproof;
        if node.proof == oldProof && node.disproof == oldDisproof {
            return node;
        }
        if node.depth == 0 {
            return node;
        }
        node = node.parent;
    }
}

fn generate_children(node: &mut Node) {
    node.children = vec![];
}
