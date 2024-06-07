use bevy::prelude::*;

#[derive(Component)]
pub struct GameComponent;

#[derive(Component, Debug, Default, Clone)]
pub struct Flag {
    pub(crate) cell: Option<Cell>,
}

#[derive(Component)]
pub struct Visible;

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Cell {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn is_near(&self, other: &Self) -> bool {
        (self.x == other.x + 1 || (other.x > 0 && self.x == other.x - 1) || self.x == other.x)
            && (self.y == other.y + 1
                || (other.y > 0 && self.y == other.y - 1)
                || self.y == other.y)
    }
}
