mod gomoku;

const BLOCK: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
const ELLY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (1, 2)];

fn main() {
    let _x = gomoku::play(6, &mut BLOCK.to_vec(), &mut ELLY.to_vec());
}
