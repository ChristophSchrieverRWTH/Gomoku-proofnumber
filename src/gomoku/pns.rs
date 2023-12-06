#![allow(unused)]
use std::rc::Rc;
type Turn = (i32, i32);

pub struct PNS {
    path: Vec<usize>,
}

impl PNS {
    // fn get_parent<'a>(&'a mut self, root: &'a mut Node) -> &'a mut Node {
    //     let mut current = root;
    //     let mut next = Some(&mut current.children[self.path[0]]);
    //     for i in 1..self.path.len() - 1 {
    //         current = next.take().unwrap();
    //         let child = &mut current.children[self.path[i]];
    //         next = Some(child);
    //     }
    //     current
    // }

    pub fn get_parent<'root>(self, root: &'root mut Node) -> (&'root mut Node, PNS) {
        let mut current = &mut root.children[self.path[0]];
        for i in 1..self.path.len() - 1 {
            let child = &mut current.children[self.path[i]];
            current = child;
        }
        (current, self)
    }

    pub fn get_node(&self) -> &mut Node {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    depth: usize,
    proof: i32,
    disproof: i32,
    expanded: bool,
    state: Status,
    node_type: NodeType,
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
    pub fn new() -> Self {
        Node {
            depth: 0,
            proof: 1,
            disproof: 1,
            expanded: false,
            state: Status::Unknown,
            node_type: NodeType::AND,
            children: vec![],
        }
    }
}

pub fn pns(mut root: Node) {
    evaluate(&mut root);
    set_numbers(&mut root);
    let mut current: &mut Node = &mut root;
    let mut most_proving: &mut Node;
    let mut pns = PNS { path: vec![] };
    while root.proof != 0 && root.disproof != 0 {
        most_proving = select_most_proving(current);
        expand(most_proving);
        (current, pns) = update_ancestors(most_proving, &mut root, pns);
    }
}

pub fn evaluate(node: &mut Node) {}

pub fn set_numbers(node: &mut Node) {
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

pub fn select_most_proving(node: &mut Node) -> &mut Node {
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

pub fn expand(node: &mut Node) {
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

pub fn update_ancestors<'a>(
    node: &'a mut Node,
    root: &'a mut Node,
    mut pns: PNS,
) -> (&'a mut Node, PNS) {
    let mut node = node;
    loop {
        let old_proof = node.proof;
        let old_disproof = node.disproof;
        if node.proof == old_proof && node.disproof == old_disproof {
            return (node, pns);
        }
        if node.depth == 0 {
            return (node, pns);
        }
        (node, pns) = PNS::get_parent(pns, root);
    }
}

pub fn generate_children(node: &mut Node) {
    node.children = vec![];
}
