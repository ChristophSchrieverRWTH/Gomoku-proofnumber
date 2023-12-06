mod gomoku;

const _STUB: [(i32, i32); 2] = [(0, 0), (1, 0)];
const _CORNER: [(i32, i32); 3] = [(0, 0), (1, 0), (1, 1)];
const _BLOCK: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
const _ELLY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (1, 2)];
const _LONGY: [(i32, i32); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];

fn main() {
    // let _x = gomoku::_play(2, &mut BLOCK.to_vec(), &mut STUB.to_vec());
    // let v = gomoku::_simulate_alphabeta(4, &mut _STUB.to_vec(), &mut _LONGY.to_vec());
    // let v = gomoku::_simulate_minmax(5, &mut _LONGY.to_vec(), &mut _LONGY.to_vec());
    // gomoku::test(5, &mut _LONGY.to_vec(), &mut _LONGY.to_vec());
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData {
    first_edge: Option<EdgeIndex>,
}

pub struct EdgeData {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_edge: None });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
    }
}
