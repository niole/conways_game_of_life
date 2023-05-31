pub mod utils;

use rand::prelude::*;
use std::convert::TryInto;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const BOARD_SIZE: usize = 1296;
const ROW_SIZE: u8 = 36;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Game {
    board: Vec<Cell>,
    row_size: u8,
}

struct Update {
    index: usize,
    value: Cell,
}

fn get_index(row_size: u8, row: u8, col: u8) -> usize {
    return usize::from(row*row_size + col);
}

fn get_xy(index: usize, row_size: u8) -> (i8, i8) {
    let x: i8 = i8::from(index as i8 / row_size as i8);
    let y: i8 = i8::from(index as i8 % row_size as i8);
    return (x, y);
}

#[wasm_bindgen]
impl Game {

    pub fn row_size(&self) -> u8 {
        return self.row_size;
    }

    pub fn board(&self) -> *const Cell {
        return self.board.as_ptr();
    }

    /**
     * Creates random game
     */
    pub fn new() -> Game {
        let mut board: Vec<Cell> = Vec::with_capacity(BOARD_SIZE.try_into().unwrap());
        let mut rng = rand::thread_rng();
        let mut i: usize = 0;
        while i < BOARD_SIZE {
            if rng.gen::<f64>().round() == 1.0 {
                board.push(Cell::Alive);
            } else {
                board.push(Cell::Dead);
            }
            i += 1;
        }
        return Game {
            board,
            row_size: ROW_SIZE
        };
    }

    /**
     * renders the boards state as an html string of a number grid
     */
    pub fn render(&mut self) -> String {
        return self.to_string();
    }

    /**
     * Updates the board state
     */
    pub fn on_tick(&mut self) {
        self.apply_updates();
    }

    fn apply_updates(&mut self) {
        let updates = self.generate_updates();
        for u in updates {
            self.board[u.index] = u.value;
        }
    }

    pub fn get_alive_neighbor_count(&self, index: usize) -> u8 {
        let offsets: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let mut count: u8 = 0;

        let (startx, starty) = get_xy(index, self.row_size);
        let rowsize: i8 = self.row_size as i8;

        for (x, y) in offsets {
            let new_coord = (startx + x, starty + y);
            if new_coord.0 >= 0 && new_coord.0 < rowsize && new_coord.1 >= 0 && new_coord.1 < rowsize {
                if self.board[get_index(self.row_size, new_coord.0 as u8, new_coord.1 as u8)] == Cell::Alive {
                    count += 1;
                }
            }
        }
        return count;
    }

    /**
     * Generates updates to apply to board
     */
    fn generate_updates(&self) -> Vec<Update> {
        let mut updates: Vec<Update> = vec![];
        for item in self.board.iter().enumerate() {
            let (i, c): (usize, &Cell) = item;
            let live_n_count = self.get_alive_neighbor_count(i);
            let index = i;

            if *c == Cell::Dead && live_n_count == 3 {
                // dead cell becomes alive
                updates.push(Update { index, value: Cell::Alive });
            }

            if *c == Cell::Alive {
                 if live_n_count < 2 {
                     updates.push(Update { index, value: Cell::Dead });
                 } else if live_n_count == 2 || live_n_count == 3  {
                     // live on
                 } else if live_n_count > 3 {
                     // overpopulated cell dies
                     updates.push(Update { index, value: Cell::Dead });
                 }
            }
        }
        return updates;
    }

}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self);
        return Ok(());
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.board.iter().enumerate() {
            let (i, c): (usize, &Cell) = item;
            if i != 0 && 0 == i%4 {
                write!(f, "<br/>")?;
            }
            write!(f, "{}", c)?;
        }
        return Ok(());
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.board.len() != other.board.len() {
            return false;
        }

        for (i, _) in self.board.iter().enumerate() {
            if self.board[i] != other.board[i] {
                return false;
            }
        }
        return true;
    }
}
impl Eq for Game {}
