use self::game::Board;
use self::pns::*;
use self::tree::Tree;
use crate::gomoku::game::Tile;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::num::ParseIntError;

mod game;
mod graph;
mod pns;
mod tree;
pub enum _Error {
    IllegalSize,
}

pub fn test(size: i32, shape1: &mut Vec<(i32, i32)>, shape2: &mut Vec<(i32, i32)>) {
    let mut root = Node::new();
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn _play(
    size: i32,
    shape1: &mut Vec<(i32, i32)>,
    shape2: &mut Vec<(i32, i32)>,
) -> Result<(), _Error> {
    if size < 0 {
        return Err(_Error::IllegalSize);
    }
    let mut board = Board::setup(size, shape1, shape2);
    println!("");
    while !board.is_over() {
        let announce = format!(
            "\nIt is player {}'s turn to move: \n
---------------------------------\n",
            board.player_to_move().to_string()
        );
        println!("{announce}");
        println!("{}", board.to_string());
        let mut clean = false;
        let mut x = -1;
        let mut y = -1;
        while !clean {
            let mut input = String::new();
            while io::stdin().read_line(&mut input).is_err() {
                println!("\nIOError occurred! Please try different input:\n");
            }
            let trimmed: Vec<&str> = input.trim().split_whitespace().collect();
            if trimmed.len() != 2 {
                println!(
                    "\nNumber of arguments incorrect. Expected 2, found {}. Please try different input:\n",
                    trimmed.len()
                );
                continue;
            }
            let attempt: Vec<Result<i32, ParseIntError>> = trimmed
                .iter()
                .map(|str| i32::from_str_radix(&str, 10))
                .collect();
            if attempt
                .iter()
                .any(|res: &Result<i32, ParseIntError>| res.is_err())
            {
                println!("\nInput could not be converted to number. Please try different input:\n");
                continue;
            }
            let numbered: Vec<i32> = attempt.iter().map(|res| *res.as_ref().unwrap()).collect();
            if numbered.iter().any(|num| num >= &size) {
                println!("\nSize too large, please chose smaller coordinates! Please try different input:\n");
                continue;
            }
            x = numbered[0];
            y = numbered[1];
            clean = true;
        }
        let place = board._place_play(x, y);
        if place.is_err() {
            println!(
                "\nUnexpected Error occurred: {:?}. Please try different input:\n",
                place
            );
            continue;
        }
    }
    match board.winner() {
        Tile::Empty => println!("\nThe game ended a draw.\n"),
        Tile::One => println!("\nPlayer one wins!\n"),
        Tile::Two => println!("\nPlayer two wins!\n"),
    }
    println!("{}", board.to_string());
    Ok(())
}

pub fn _simulate_minmax(
    size: i32,
    shape1: &mut Vec<(i32, i32)>,
    shape2: &mut Vec<(i32, i32)>,
) -> &'static str {
    let mut board = Board::setup(size, shape1, shape2);
    board.place_proof((size / 2) as i32, (size / 2) as i32);
    let mut tree = Tree::new(size);
    tree.legal
        .insert(((size / 2) as i32, (size / 2) as i32), false);
    let value = tree._minimax(&mut board, true, 0);
    match value {
        1 => "Player one wins.",
        2 => "Player two wins.",
        0 => "Draw",
        _ => "",
    }
}

pub fn _simulate_alphabeta(
    size: i32,
    shape1: &mut Vec<(i32, i32)>,
    shape2: &mut Vec<(i32, i32)>,
) -> &'static str {
    let mut board = Board::setup(size, shape1, shape2);
    board.place_proof((size / 2) as i32, (size / 2) as i32);
    let mut tree = Tree::new(size);
    tree.legal
        .insert(((size / 2) as i32, (size / 2) as i32), false);
    let value = tree._alphabeta(&mut board, true, -2, 2);
    match value {
        1 => "Player one wins.",
        2 => "Player two wins.",
        0 => "Draw",
        _ => "wat",
    }
}
