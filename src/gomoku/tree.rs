use crate::gomoku::game::*;
use std::cmp;
use std::{collections::HashMap, rc::Rc};

pub struct Tree {
    pub legal: HashMap<(i32, i32), bool>,
    pub moves: Vec<(i32, i32)>,
}

impl Tree {
    pub fn new(size: i32) -> Self {
        let mut legal = HashMap::new();
        for i in 0..size {
            for j in 0..size {
                legal.insert((i, j), true);
            }
        }
        Tree {
            legal,
            moves: vec![],
        }
    }

    pub fn evaluate(&mut self, board: &Board) -> i32 {
        match board.winner() {
            Tile::One => 1,
            Tile::Two => -1,
            Tile::Empty => 0,
        }
    }

    pub fn calculate(&mut self, board: &mut Board, x_cord: i32, y_cord: i32) -> Board {
        let mut new_board = Board {
            field: board.field.clone(),
            shapes1: Rc::clone(&board.shapes1),
            shapes2: Rc::clone(&board.shapes2),
            ..*board
        };
        let res = new_board.place(x_cord, y_cord);
        match res.is_err() {
            true => panic!("Board place returned error."),
            false => new_board,
        }
    }

    pub fn expand(&self) -> HashMap<(i32, i32), bool> {
        let mut hm = HashMap::new();
        for ((l1, l2), answer) in &self.legal {
            if *answer {
                hm.insert((*l1, *l2), true);
            }
        }
        hm
    }

    pub fn minimax(&mut self, board: &mut Board, maximizer: bool, depth: usize) -> i32 {
        if board.is_over() {
            return self.evaluate(&board);
        }
        let mut best_val;
        let d = depth + 1;
        if maximizer {
            best_val = -2;
            self.legal = self.expand();
            for ((x_cord, y_cord), _) in self.expand() {
                if depth == 1 {
                    println!("Searched a complete subtree!");
                }
                let mut child = self.calculate(board, x_cord, y_cord);
                self.legal.insert((x_cord, y_cord), false);

                let current_val = self.minimax(&mut child, false, d);
                self.legal.insert((x_cord, y_cord), true);
                if current_val == 1 {
                    return 1;
                }
                best_val = cmp::max(best_val, current_val);
            }
        } else {
            best_val = 2;
            self.legal = self.expand();
            for ((x_cord, y_cord), _) in self.expand() {
                if depth == 1 {
                    println!("Searched a complete subtree!");
                }
                let mut child = self.calculate(board, x_cord, y_cord);
                self.legal.insert((x_cord, y_cord), false);
                let current_val = self.minimax(&mut child, true, d);
                self.legal.insert((x_cord, y_cord), true);
                if current_val == -1 {
                    return -1;
                }

                best_val = cmp::min(best_val, current_val);
            }
        }
        best_val
    }

    pub fn alphabeta(&mut self, board: &mut Board, maximizer: bool, alpha: i32, beta: i32) -> i32 {
        if board.is_over() {
            return self.evaluate(&board);
        }
        let mut val;
        if maximizer {
            val = -2;
            self.legal = self.expand();
            let mut new_alpha = alpha;
            for ((x_cord, y_cord), _) in self.expand() {
                let mut child = self.calculate(board, x_cord, y_cord);
                self.legal.insert((x_cord, y_cord), false);
                val = self.alphabeta(&mut child, false, new_alpha, beta);
                self.legal.insert((x_cord, y_cord), true);
                if val > beta {
                    break;
                }
                new_alpha = cmp::max(alpha, val);
            }
        } else {
            val = 2;
            self.legal = self.expand();
            let mut new_beta = beta;
            for ((x_cord, y_cord), _) in self.expand() {
                let mut child = self.calculate(board, x_cord, y_cord);
                self.legal.insert((x_cord, y_cord), false);
                val = self.alphabeta(&mut child, true, alpha, new_beta);
                self.legal.insert((x_cord, y_cord), true);
                if val < alpha {
                    break;
                }

                new_beta = cmp::min(beta, val);
            }
        }
        val
    }
}
