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

#[macro_export]
macro_rules! embedded_asset {
    ($app: ident, $path: expr) => {{
        embedded_asset!($app, "src", $path)
    }};

    ($app: ident, $source_path: expr, $path: expr) => {{
        embedded_asset!($app, $source_path, $path, $path)
    }};

    ($app: ident, $source_path: expr, $path: expr, $renamed: expr) => {{
        let mut embedded = $app
            .world
            .resource_mut::<bevy::asset::io::embedded::EmbeddedAssetRegistry>();
        let path = bevy::asset::embedded_path!($source_path, $renamed);
        let watched_path = bevy::asset::io::embedded::watched_path(file!(), $path);
        embedded.insert_asset(watched_path, &path, include_bytes!($path));
    }};
}
