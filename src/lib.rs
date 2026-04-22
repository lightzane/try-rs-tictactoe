pub mod engine;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<Cell>,
    pub current_player: Player,
}

impl Board {
    pub fn new(rows: usize, cols: usize, starter: Option<Player>) -> Self {
        Self {
            rows,
            cols,
            grid: vec![Cell::Empty; rows * cols],
            // Use unwrap_or to default to X if None is provided
            current_player: starter.unwrap_or(Player::X),
        }
    }

    // Helper to get the index in the flat Vec
    pub fn get_index(&self, row: usize, col: usize) -> usize {
        (row * self.cols) + col
    }

    pub fn set_cell(&mut self, row: usize, col: usize, player: Player) {
        let idx = self.get_index(row, col);
        self.grid[idx] = Cell::Occupied(player);
    }

    pub fn from_string(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let layout = parts[0]; // e.g. 3/1o1/3
        let side = parts[1]; // e.g. x

        let rows_raw: Vec<&str> = layout.split('/').collect();
        let row_count = rows_raw.len();
        let col_count = 3; // for now

        let mut grid = vec![Cell::Empty; row_count * col_count];

        for (r, row_str) in rows_raw.iter().enumerate() {
            let mut c = 0;
            for ch in row_str.chars() {
                if let Some(digit) = ch.to_digit(10) {
                    c += digit as usize // skip empty cells
                } else {
                    let player = if ch == 'x' { Player::X } else { Player::O };
                    let idx = (r * col_count) + c;
                    grid[idx] = Cell::Occupied(player);
                    c += 1;
                }
            }
        }

        let next_to_move = if side == "x" { Player::X } else { Player::O };

        Self {
            rows: row_count,
            cols: col_count,
            grid,
            current_player: next_to_move,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            let mut empty_count = 0;

            for c in 0..self.cols {
                match self.grid[r * self.cols + c] {
                    Cell::Empty => empty_count += 1,
                    Cell::Occupied(p) => {
                        if empty_count > 0 {
                            write!(f, "{empty_count}")?;
                            empty_count = 0;
                        }
                        write!(f, "{}", if p == Player::X { 'x' } else { 'o' })?;
                    }
                }
            }

            if empty_count > 0 {
                write!(f, "{empty_count}")?
            }

            if r < self.rows - 1 {
                write!(f, "/")?
            }
        }

        let next_to_move = if self.current_player == Player::X {
            'x'
        } else {
            'o'
        };

        write!(f, " {next_to_move}")
    }
}

impl Board {
    pub fn count_sequence(&self, row: usize, col: usize, dr: i32, dc: i32) -> usize {
        let p = self.grid[self.get_index(row, col)];
        let mut count = 1;

        for i in 1..3 {
            let nr = row as i32 + (dr * i);
            let nc = col as i32 + (dc * i);

            if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                if self.grid[self.get_index(nr as usize, nc as usize)] == p {
                    count += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        count
    }

    pub fn check_winner(&self) -> Option<Player> {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

        for r in 0..self.rows {
            for c in 0..self.cols {
                if let Cell::Occupied(p) = self.grid[self.get_index(r, c)] {
                    for (dr, dc) in directions {
                        if self.count_sequence(r, c, dr, dc) >= 3 {
                            return Some(p);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn get_legal_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = vec![];

        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[self.get_index(r, c)] == Cell::Empty {
                    moves.push((r, c));
                }
            }
        }

        moves
    }
}

impl Board {
    pub fn make_move(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.grid[idx] = Cell::Occupied(self.current_player);
        self.toggle_player();
    }

    pub fn undo_move(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.grid[idx] = Cell::Empty;
        self.toggle_player();
    }

    fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Board {
    pub fn is_draw(&self) -> bool {
        self.check_winner().is_none() && self.get_legal_moves().is_empty()
    }

    pub fn is_game_over(&self) -> bool {
        self.check_winner().is_some() || self.get_legal_moves().is_empty()
    }
}

impl Board {
    pub fn print_visual(&self) {
        println!("\n      0   1   2  "); // Column headers
        println!("    -------------");
        for r in 0..self.rows {
            print!(" {} |", r); // Row headers
            for c in 0..self.cols {
                let cell = match self.grid[self.get_index(r, c)] {
                    Cell::Empty => " ",
                    Cell::Occupied(Player::X) => "X",
                    Cell::Occupied(Player::O) => "O",
                };
                print!(" {} |", cell);
            }
            println!("\n    -------------");
        }
    }
}

// ----- WASM EXPORTS -----------------------------------------------------------

use crate::engine::Difficulty;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_tictactoe(board_string: &str, difficulty: &str) -> Vec<usize> {
    // 1. Turn string from JS into Board struct
    let mut board = Board::from_string(board_string);

    // 2. Convert string difficulty into enum
    let level = match difficulty {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        _ => Difficulty::Hard,
    };

    // 3. Get best move from engine
    if let Some((r, c)) = engine::get_best_move(&mut board, level) {
        // Return simple array [row, col] that JS understands
        return vec![r, c];
    }

    // If no moves are left, return empty
    vec![]
}

#[wasm_bindgen]
pub fn check_game_status(board_string: &str) -> String {
    let board = Board::from_string(board_string);

    if let Some(winner) = board.check_winner() {
        return match winner {
            Player::X => "X_WINS".to_string(),
            Player::O => "O_WINS".to_string(),
        };
    }

    if board.is_draw() {
        return "DRAW".to_string();
    }

    "ONGOING".to_string()
}
