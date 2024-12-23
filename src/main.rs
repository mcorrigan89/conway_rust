use raylib::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn new() -> Universe {
        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
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

    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

fn main() {
    let mut universe = Universe::new();
    let height = universe.height();
    let width = universe.width();

    let cell_size: i32 = 6;
    let (mut rl, thread) = raylib::init()
        .size(
            (width * cell_size) + (cell_size * 5),
            (height * cell_size) + (cell_size * 5),
        )
        .title("Conway's Game of Life!")
        .build();

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for i in 0..height {
            d.draw_line(
                i * (cell_size + 1) + 1,
                0,
                i * (cell_size + 1) + 1,
                (cell_size + 1) * height + 1,
                Color::BLACK,
            );
        }

        for i in 0..height {
            d.draw_line(
                0,
                i * (cell_size + 1) + 1,
                (cell_size + 1) * width + 1,
                i * (cell_size + 1) + 1,
                Color::BLACK,
            );
        }

        universe.tick();

        let cells = universe.cells();

        for row in 0..height {
            for col in 0..width {
                let idx = universe.get_index(row, col);

                let c = cells[idx];

                match c {
                    Cell::Alive => {
                        d.draw_rectangle(
                            col * (cell_size + 1) + 1,
                            row * (cell_size + 1) + 1,
                            cell_size,
                            cell_size,
                            Color::GRAY,
                        );
                    }
                    Cell::Dead => {
                        d.draw_rectangle(
                            col * (cell_size + 1) + 1,
                            row * (cell_size + 1) + 1,
                            cell_size,
                            cell_size,
                            Color::WHITESMOKE,
                        );
                    }
                }
            }
        }
    }
}
