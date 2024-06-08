use bevy::prelude::*;

#[derive(Component)]
pub struct GameComponent;

#[derive(Component, Debug, Default, Clone)]
pub struct Flag {
    pub(crate) cell: Option<Cell>,
}

#[derive(Component)]
pub struct Visible;

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Cell {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Cell {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn is_near(&self, other: &Self) -> bool {
        (self.x == other.x + 1.0 || (other.x > 0.0 && self.x == other.x - 1.0) || self.x == other.x)
            && (self.y == other.y + 1.0
                || (other.y > 0.0 && self.y == other.y - 1.0)
                || self.y == other.y)
    }
}
