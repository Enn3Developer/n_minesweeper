use crate::game::components::Cell;
use crate::get_path;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;

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

    pub fn generate(&mut self, mut bombs: u32, no_bomb_zone: Option<Cell>) {
        let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
        while bombs > 0 {
            let x = rng.sample(Uniform::new(0, self.grid_width));
            let y = rng.sample(Uniform::new(0, self.grid_height));
            if self.is_bomb(x as f32, y as f32)
                || no_bomb_zone
                    .as_ref()
                    .is_some_and(|cell| cell.x == x as f32 && cell.y == y as f32)
            {
                continue;
            }
            self.bombs.push(Cell::new(x as f32, y as f32));
            bombs -= 1;
        }
    }

    pub fn is_bomb(&self, x: f32, y: f32) -> bool {
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
            (x / self.width as f32 * self.grid_width as f32).floor(),
            (y / self.height as f32 * self.grid_height as f32).floor(),
        )
    }

    pub fn grid_to_global(&self, cell: &Cell) -> (f32, f32) {
        (
            (cell.x + 0.5) * self.width as f32 / self.grid_width as f32,
            (cell.y + 0.5) * self.height as f32 / self.grid_height as f32,
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

#[derive(Resource, Default)]
pub struct ClearingCells {
    pub(crate) cells: VecDeque<(Entity, Cell)>,
}

#[derive(Resource, Default)]
pub struct ChangeCells {
    pub(crate) cells: Vec<Cell>,
}

#[derive(Resource, Default)]
pub struct GameData {
    images: Vec<Handle<Image>>,
    text_styles: Vec<TextStyle>,
}

impl GameData {
    pub fn setup(&mut self, server: &AssetServer) {
        self.images
            .push(server.load(get_path("textures/closed.png")));
        self.images.push(server.load(get_path("textures/open.png")));
        self.images
            .push(server.load(get_path("textures/atlas.png")));
        let style = TextStyle {
            color: Color::BLACK,
            font_size: 24.0,
            ..default()
        };
        self.text_styles.push(style);
        let style = TextStyle {
            color: Color::BLACK,
            font_size: 24.0,
            font: server.load(get_path("fonts/NotoEmoji.ttf")),
        };
        self.text_styles.push(style);
    }

    pub fn closed_cell(&self) -> Handle<Image> {
        self.images[0].clone()
    }

    pub fn open_cell(&self) -> Handle<Image> {
        self.images[1].clone()
    }

    pub fn atlas(&self) -> Handle<Image> {
        self.images[2].clone()
    }

    pub fn normal_text(&self) -> TextStyle {
        self.text_styles[0].clone()
    }

    pub fn flag_text(&self) -> TextStyle {
        self.text_styles[1].clone()
    }
}

#[derive(Resource)]
pub struct NTimer(pub(crate) Timer);
