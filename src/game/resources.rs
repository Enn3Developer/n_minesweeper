use crate::game::components::Cell;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Resource)]
pub struct Grid {
    bombs: Vec<Cell>,
    grid_width: u32,
    grid_height: u32,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(grid_width: u32, grid_height: u32, width: u32, height: u32) -> Self {
        Grid {
            bombs: Vec::with_capacity(64),
            grid_width,
            grid_height,
            width,
            height,
        }
    }

    pub fn generate(&mut self, mut bombs: u32) {
        let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
        while bombs > 0 {
            let x = rng.sample(Uniform::new(0, self.grid_width));
            let y = rng.sample(Uniform::new(0, self.grid_height));
            if self.is_bomb(x, y) {
                continue;
            }
            self.bombs.push(Cell::new(x, y));
            bombs -= 1;
        }
    }

    pub fn is_bomb(&self, x: u32, y: u32) -> bool {
        for bomb in &self.bombs {
            if bomb.x == x && bomb.y == y {
                return true;
            }
        }
        false
    }

    pub fn is_bomb_cell(&self, cell: &Cell) -> bool {
        self.is_bomb(cell.x, cell.y)
    }

    pub fn bombs(&self) -> &[Cell] {
        &self.bombs
    }

    pub fn global_to_grid(&self, x: f32, y: f32) -> Cell {
        Cell::new(
            (x / self.width as f32 * self.grid_width as f32).floor() as u32,
            (y / self.height as f32 * self.grid_height as f32).floor() as u32,
        )
    }

    pub fn grid_to_global(&self, cell: &Cell) -> (f32, f32) {
        (
            (cell.x as f32 + 0.5) * self.width as f32 / self.grid_width as f32,
            (cell.y as f32 + 0.5) * self.height as f32 / self.grid_height as f32,
        )
    }
}

#[derive(Resource, Default)]
pub struct TextGrid {
    texts: Vec<Cell>,
}

impl TextGrid {
    pub fn add(&mut self, cell: Cell) {
        self.texts.push(cell);
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.texts.contains(cell)
    }
}
