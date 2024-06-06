use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use n_minesweeper::{check_cell, grid_setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, grid_setup)
        .add_systems(
            Update,
            check_cell.run_if(input_just_pressed(MouseButton::Left)),
        )
        .run();
}
