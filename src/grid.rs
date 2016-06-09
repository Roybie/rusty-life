extern crate rand;

use rand::Rng;

const SCALE:usize = 4;
const WIDTH:usize = 800 / SCALE;
const HEIGHT :usize = 600 / SCALE;
const SIZE :usize= WIDTH * HEIGHT;

pub struct Grid {
    cells: Box<[bool]>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: vec![false;SIZE].into_boxed_slice(),
        }
    }

    pub fn scale(&self) -> usize {
        SCALE
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    //pub fn height(&self) -> usize {
        //HEIGHT
    //}

    pub fn seed(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = rand::thread_rng().gen();
        }
    }

    pub fn get_cells(&self) -> &Box<[bool]> {
        &self.cells
    }

    pub fn update(&mut self) {
        let old = self.cells.clone();

        for i in 0..old.len() {
            let neighbours = vec![
                old.get(i-1),
                old.get(i+1),
                old.get(i-WIDTH),
                old.get(i+WIDTH),
                old.get(i-(WIDTH+1)),
                old.get(i-(WIDTH-1)),
                old.get(i+(WIDTH+1)),
                old.get(i+(WIDTH-1)),
            ].iter().filter(|o| o.is_some() && *o.unwrap()).count();

            self.cells[i] = match neighbours {
                2 => self.cells[i],
                3 => true,
                _ => false,
            };
        }
    }
}
