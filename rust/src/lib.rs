mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

const DEFAULT_SPEED: u32 = 1;

#[wasm_bindgen]
pub struct Universe {
    speed: u32,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    /**
     * Returns the index of the cell in the universe
     */
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /**
     * Returns the number of live neighbors
     */
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                };

                let neighbor_row = (delta_row + row) % self.height;
                let neighbow_column = (delta_column + column) % self.width;
                let neighbor_idx = self.get_index(neighbor_row, neighbow_column);
                count += self.cells[neighbor_idx] as u8;
            }
        }
        count
    }

    /**
     * Returns a vector reference of the entire universe
     */
    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /**
     * List reference of tuples (row, column) order
     */
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let index = self.get_index(row, col);
            self.cells[index] = Cell::Alive
        }
    }

    /**
     * Get random cells universe
     */
    pub fn get_random_universe(width: &u32, height: &u32) -> Vec<Cell> {
        (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect()
    }
}

#[wasm_bindgen]
impl Universe {
    /**
     * Process the universe to the next generation
     */
    pub fn tick(&mut self) {
        for _ in 0..self.speed {
            let mut next = self.cells.clone();
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Cell::Dead, 3) => Cell::Alive,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
            self.cells = next;
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..height * self.width).map(|_i| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    /**
     * Returns a pointer to the cells
     */
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let index = self.get_index(row, column);
        self.cells[index].toggle();
    }

    pub fn reset(&mut self) {
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn random(&mut self) {
        self.cells = Universe::get_random_universe(&self.width, &self.height);
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        let cells = Universe::get_random_universe(&width, &height);

        Universe {
            speed: DEFAULT_SPEED,
            width,
            height,
            cells,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.reset()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
