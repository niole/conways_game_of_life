pub mod utils;

use std::convert::TryInto;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}

#[wasm_bindgen]
pub struct Game {
    board: [u8; 16],
    row_size: u8,
}

struct Update {
    index: usize,
    value: u8,
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
    pub fn new_board(opt_board: Vec<u8>) -> Result<Game, String> {
        if opt_board.len() != 16 {
            return Err(String::from("bad length"));
        }

        let board = opt_board.try_into()
            .unwrap_or_else(
                |opt_board: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 16, opt_board.len())
            );
        let row_size = 4;
        return Ok(Game {
            board,
            row_size
        });
    }
    pub fn new() -> Game {
        let row_size = 4;
        let mut default_board = [0; 16];
        default_board[get_index(row_size, 0, 0)] = 1;
        default_board[get_index(row_size, 1, 0)] = 1;
        default_board[get_index(row_size, 2, 0)] = 1;
        return Game {
            board : default_board,
            row_size
        };
    }

    pub fn render(&mut self) -> String {
        return self.to_string();
    }

    pub fn on_tick(&mut self) {
        self.apply_updates();
    }

    fn apply_updates(&mut self) {
        let updates = self.generate_updates();
        for u in updates {
            self.board[u.index] = u.value;
        }
    }

    fn get_alive_neighbor_count(&self, index: usize) -> u8 {
        // TODO check above, sides, bottom for 1s
        let offsets: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let mut count: u8 = 0;

        let (startx, starty) = get_xy(index, self.row_size);
        let rowsize: i8 = self.row_size as i8;

        for (x, y) in offsets {
            let new_coord = (startx + x, starty + y);
            if new_coord.0 >= 0 && new_coord.0 < rowsize && new_coord.1 >= 0 && new_coord.1 < rowsize {
                if get_index(self.row_size, new_coord.0 as u8, new_coord.1 as u8) == 1 {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn generate_updates(&self) -> Vec<Update> {
        let mut updates: Vec<Update> = vec![];
        for item in self.board.iter().enumerate() {
            let (i, c): (usize, &u8) = item;
            let live_n_count = self.get_alive_neighbor_count(i);
            let index = i;

            if live_n_count < 2 && 1 == *c {
                // live cell dies from underpopulation
                let value = 0;
                updates.push(Update { index, value });
            } else if live_n_count == 2 || live_n_count == 3 && 1 == *c {
                // live cell lives on
            } else if live_n_count > 3 && 1 == *c {
                // overpopulated cell dies
                let value = 0;
                updates.push(Update { index, value });
            } else if live_n_count == 3 && 0 == *c {
                // dead cell becomes alive
                let value = 1;
                updates.push(Update { index, value });
            }
        }
        return updates;
    }

}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.board.iter().enumerate() {
            let (i, c): (usize, &u8) = item;
            if i != 0 && 0 == i%4 {
                write!(f, "<br/>")?;
            }
            write!(f, "{}", c)?;
        }
        return Ok(());
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.board.iter().enumerate() {
            let (i, c): (usize, &u8) = item;
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
