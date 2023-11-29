use self::game::Board;
use crate::gomoku::game::Tile;
use std::io;
use std::num::ParseIntError;

mod game;

pub fn play(
    size: i32,
    shape1: &mut Vec<(i32, i32)>,
    shape2: &mut Vec<(i32, i32)>,
) -> Result<(), Error> {
    if size < 0 {
        return Err(Error::IllegalSize);
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
        let place = board.place(x, y);
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

pub enum Error {
    IllegalSize,
}
