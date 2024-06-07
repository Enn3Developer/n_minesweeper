#![feature(trivial_bounds)]

pub mod end;
pub mod game;
pub mod menu;

use crate::end::End;
use crate::game::Game;
use crate::menu::Menu;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct NMines;

impl PluginGroup for NMines {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Game)
            .add(End)
            .add(Menu)
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
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
