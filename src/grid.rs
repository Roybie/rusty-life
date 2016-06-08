extern crate rand;

use rand::Rng;

const scale:usize = 4;
const width:usize = 800 / scale;
const height :usize = 600 / scale;
const size :usize= width * height;

pub struct Grid {
    cells: Box<[bool]>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: vec![false;size].into_boxed_slice(),
        }
    }

    pub fn scale(&self) -> usize {
        scale
    }

    pub fn width(&self) -> usize {
        width
    }

    pub fn height(&self) -> usize {
        height
    }

    pub fn seed(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = rand::thread_rng().gen_range(0, 2) == 1;
        }
    }

    pub fn get_cells(&self) -> &Box<[bool]> {
        &self.cells
    }

    pub fn update(&mut self) {
        let old = self.cells.clone();

        for i in 0..old.len() {
            let mut neighbours = 0;
            // -1
            if i % width > 0 && old[i-1] {
                neighbours += 1;
            }
            // +1
            if i % width < width - 1 && old[i+1] {
                neighbours += 1;
            }
            // -20
            if i > width - 1 && old[i-width] {
                neighbours += 1;
            }
            // +20
            if i < size - width - 1 && old[i+width] {
                neighbours += 1;
            }
            // -21
            if i % width > 0 && i > width && old[i-(width + 1)] {
                neighbours += 1;
            }
            // -19
            if i % width < (width - 1) && i > (width - 1) && old[i-(width-1)] {
                neighbours += 1;
            }
            // +19
            if i < size - width - 1 && i % width > 0 && old[i+(width - 1)] {
                neighbours += 1;
            }
            // +21
            if i % width > 0 && i < size - width - 1 && old[i + width + 1] {
                neighbours += 1;
            }
            self.cells[i] = match neighbours {
                2 => self.cells[i],
                3 => true,
                _ => false,
            };
        }
    }
}
