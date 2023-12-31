mod gomoku;

const _STUB: [(i32, i32); 2] = [(0, 0), (1, 0)];
const _CORNER: [(i32, i32); 3] = [(0, 0), (1, 0), (1, 1)];
const _BLOCK: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
const _ELLY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (1, 2)];
const _SKINNY: [(i32, i32); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];

fn main() {
    // let _x = gomoku::_play(3, &mut _BLOCK.to_vec(), &mut _STUB.to_vec());
    // let v = gomoku::_simulate_alphabeta(4, &mut _STUB.to_vec(), &mut _LONGY.to_vec());
    // let v = gomoku::_simulate_minmax(5, &mut _LONGY.to_vec(), &mut _LONGY.to_vec());
    gomoku::basic_pns(
        5,
        &mut _SKINNY.to_vec(),
        &mut _SKINNY.to_vec(),
        vec![(2,2), (2,3)],
    );
}
