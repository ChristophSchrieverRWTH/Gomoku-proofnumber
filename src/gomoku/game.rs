use std::rc::Rc;
use std::{collections::HashMap, fmt::Display, result::Result};

#[derive(Debug)]
pub struct Board {
    size: i32,
    turn: usize,
    field: HashMap<(i32, i32), Tile>,
    player_one: bool,
    game_over: bool,
    shapes: Rc<Shapes>,
    winner: Tile,
}

#[derive(Debug)]
pub enum Error {
    CordIllegalLarge,
    CordIllegalSmall,
    AlreadyOccupied,
}

#[derive(Debug)]
pub struct Shapes {
    shapes: Vec<Vec<(i32, i32)>>,
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    One,
    Two,
    Empty,
}
impl Shapes {
    pub fn new(shape: &mut Vec<(i32, i32)>) -> Self {
        let shapes = get_shapes(shape);
        Shapes { shapes }
    }
}

impl Board {
    /// Setup function only to be called once
    pub fn setup(size: i32, shape: &mut Vec<(i32, i32)>) -> Board {
        let mut field = HashMap::new();
        for i in 0..size {
            for j in 0..size {
                field.insert((i as i32, j as i32), Tile::Empty);
            }
        }
        Board {
            size,
            turn: 1,
            field,
            player_one: true,
            game_over: false,
            shapes: Rc::new(Shapes::new(shape)),
            winner: Tile::Empty,
        }
    }
    /// Places ones stone at coordinates x, y assuming it is still empty and on the board.
    ///

    pub fn place(&mut self, x_cord: i32, y_cord: i32) -> Result<(), Error> {
        if x_cord >= self.size || y_cord >= self.size {
            return Err(Error::CordIllegalLarge);
        }
        if x_cord < 0 || y_cord < 0 {
            return Err(Error::CordIllegalSmall);
        }
        if self.field.get(&(x_cord, y_cord)).unwrap() != &Tile::Empty {
            return Err(Error::AlreadyOccupied);
        }
        let active = self.player_to_move();
        self.field.insert((x_cord, y_cord), active);
        self.game_over = self.game_over(x_cord, y_cord);
        if self.game_over {
            return Ok(());
        }
        self.player_one = !self.player_one;
        Ok(())
    }

    pub fn game_over(&mut self, x_cord: i32, y_cord: i32) -> bool {
        if self.turn == self.size.pow(2) as usize {
            self.winner = Tile::Empty;
            return true;
        }
        let mut over = false;
        for shape in &self.shapes.shapes[..] {
            'outer: for (s1, s2) in shape {
                for (t1, t2) in shape {
                    let x = s1 - t1 + x_cord;
                    let y = s2 - t2 + y_cord;
                    match self.field.get(&(x, y)) {
                        None => continue 'outer,
                        Some(tile) => match (tile, self.player_one) {
                            (Tile::One, true) | (Tile::Two, false) => continue,
                            (_, _) => continue 'outer,
                        },
                    }
                }
                over = true;
                self.winner = self.player_to_move();
            }
        }
        over
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }

    pub fn player_to_move(&self) -> Tile {
        match self.player_one {
            true => Tile::One,
            false => Tile::Two,
        }
    }

    pub fn winner(&self) -> &Tile {
        &self.winner
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for i in 0..self.size {
            for j in 0..self.size {
                let c = match self
                    .field
                    .get(&(j, i))
                    .expect("All Coordinates should be at least Empty.")
                {
                    Tile::Empty => "—",
                    Tile::One => "ⵔ",
                    Tile::Two => "X",
                };
                string.push_str(c);
                string.push_str(" ");
                if j == self.size - 1 {
                    string.push_str("\n");
                }
            }
        }
        write!(f, "{}", string)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Tile::One => "one",
            Tile::Two => "two",
            Tile::Empty => "empty",
        };
        write!(f, "{}", string)
    }
}

pub fn get_shapes(shape: &mut Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let mut new_shape = vec![];
    for _i in 0..2 {
        for _j in 0..4 {
            for (sx, sy) in &mut *shape {
                let temp = *sx;
                *sx = -(*sy);
                *sy = temp;
            }
            new_shape.push(shape.clone());
        }
        for (sx, _sy) in &mut *shape {
            *sx = -(*sx);
        }
    }
    new_shape
}
