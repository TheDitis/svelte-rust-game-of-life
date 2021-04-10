mod utils;
mod timer;

extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use web_sys::console;
use timer::Timer;


macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // to time each tick and log the time in console
        let _timer = Timer::new("Universe::tick");

        // make a mutable copy of the current cells for the next tick
        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };
        {
            let _timer = Timer::new("new generation");
            // for each cell in each row
            for row in 0..self.height {
                for col in 0..self.width {
                    // get index of the current cell, and the value of that cell itself
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];

                    // get the number of live neighbors for that cell
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbors dies from underpopulation
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbors lives on to the next generation
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than 3 live neighbors dies of overpopulation
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly 3 live neighbors becomes a live cell by reproduction
                        (Cell::Dead, x) if x == 3 => Cell::Alive,
                        // All other cells remain in their current state
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
        }
        let _timer = Timer::new("free old cells");
        self.cells = next;
    }


    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }


    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn new() -> Universe {
        // now if we use the panic macro we will get
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        // set the starting state for our cells
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    //
    // pub fn render(&self) -> String {
    //     // to_string() works since we added the Display trait
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// SETTER METHODS

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn clear(&mut self) {
        for c in &mut self.cells {
            *c = Cell::Dead;
        }
    }

    pub fn invert(&mut self) {
        for c in &mut self.cells {
            c.toggle();
        }
    }

}


/// Testing methods (not exposed to wasm_bindgen)
impl Universe {
    /// Get the dead and alive values of the entire universe!
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        // for (row, col) in cells.iter().cloned() {
        //     let idx = self.get_index(row, col);
        //     self.cells[idx] = Cell::Alive;
        // }
        for c in &mut self.cells {
            *c = Cell::Dead;
        }
    }
}


/// Implementing the Display trait for Universe so we can use .to_string()
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // for each row
        for line in self.cells.as_slice().chunks(self.width as usize) {
            // for each cell, add to the formatter ◻ if dead or ◼ if alive
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            // add newline so that the output wraps for each row
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, wasm-game-of-life!".into()
}