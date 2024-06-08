pub mod end;
pub mod game;
pub mod menu;

use crate::end::End;
use crate::game::Game;
use crate::menu::Menu;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::Stopwatch;

pub struct NMines;

impl PluginGroup for NMines {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NStartup)
            .add(Game)
            .add(End)
            .add(Menu)
    }
}

pub struct NStartup;

impl Plugin for NStartup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init, setup));
    }
}

pub fn init(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.resolution.set(600.0, 600.0);
    window.resize_constraints.min_height = 600.0;
    window.resize_constraints.min_width = 600.0;
    window.title = String::from("N Mines");
}

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(600.0 / 2.0, 600.0 / 2.0, 1000.0);
    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn(camera);
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

#[derive(Resource, Default)]
pub struct NStopWatch(pub(crate) Stopwatch);

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
