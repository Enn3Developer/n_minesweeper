pub mod end;
pub mod game;
pub mod menu;

use crate::end::End;
use crate::game::Game;
use crate::menu::Menu;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use std::time::Instant;

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
        app.add_systems(Startup, (init, setup))
            .add_systems(Update, move_camera.run_if(in_state(AppState::Playing)));
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

pub fn move_camera(
    mut cameras: Query<&mut Transform, With<Camera>>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = cameras.single_mut();
    let mut movement = Vec2::ZERO;
    let speed = 250.0;
    if button_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        movement += Vec2::Y;
    }
    if button_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        movement -= Vec2::Y;
    }
    if button_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        movement -= Vec2::X;
    }
    if button_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        movement += Vec2::X;
    }

    transform.translation += (movement * speed * time.delta_seconds()).extend(0.0);
}

pub fn get_path(path: &str) -> String {
    format!("embedded://n_minesweeper/../assets/{path}")
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

#[derive(Resource)]
pub struct NStopWatch(pub(crate) Instant);

#[derive(Resource)]
pub struct GameSettings {
    pub width: u32,
    pub height: u32,
    pub bombs: u32,
    pub speed: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            width: 20,
            height: 20,
            bombs: 40,
            speed: 8,
        }
    }
}
