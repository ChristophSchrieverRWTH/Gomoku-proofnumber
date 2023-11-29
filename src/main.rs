mod gomoku;

const STUB: [(i32, i32); 2] = [(0, 0), (1, 0)];
const BLOCK: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
const ELLY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (1, 2)];

fn main() {
    let _x = gomoku::play(2, &mut BLOCK.to_vec(), &mut STUB.to_vec());
}
