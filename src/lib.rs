pub mod end;
pub mod game;

use crate::end::End;
use crate::game::Game;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct NMines;

impl PluginGroup for NMines {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(Game).add(End)
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Playing,
    End,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EndState {
    #[default]
    NotEnded,
    Win,
    Lose,
}
