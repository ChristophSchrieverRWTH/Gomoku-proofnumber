#![allow(unused)]
use std::io;
mod gomoku;

const STUB: [(i32, i32); 2] = [(0, 0), (1, 0)];
const CORNER: [(i32, i32); 3] = [(0, 0), (1, 0), (1, 1)];
const BLOCKY: [(i32, i32); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];
const ELLY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (1, 2)];
const SKINNY: [(i32, i32); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const TIPPY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (2, 1)];
const KNOBBY: [(i32, i32); 4] = [(0, 0), (1, 0), (1, 1), (2, 0)];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let trimmed: Vec<&str> = input.trim().split_whitespace().collect();
    let size = trimmed[0].to_string();
    let size = size.parse::<i32>().unwrap();
    let mut shape1 = get_shape(trimmed[1]);
    let mut shape2 = get_shape(trimmed[2]);
    let mut moves_made = vec![];
    for i in 3..trimmed.len() {
        let coordinate: Vec<&str> = trimmed[i].split(",").collect();
        let x = coordinate[0].to_string();
        let x = x.parse::<i32>().unwrap();
        let y = coordinate[1].to_string();
        let y = y.parse::<i32>().unwrap();
        moves_made.push((x, y));
    }
    gomoku::test(size, &mut shape1, &mut shape2, moves_made);
}

pub fn get_shape(input: &str) -> Vec<(i32, i32)> {
    let input = input.to_uppercase();
    match &input[..] {
        "BLOCKY" => BLOCKY.to_vec(),
        "TIPPY" => TIPPY.to_vec(),
        "ELLY" => ELLY.to_vec(),
        "SKINNY" => SKINNY.to_vec(),
        "KNOBBY" => KNOBBY.to_vec(),
        _ => vec![],
    }
}
