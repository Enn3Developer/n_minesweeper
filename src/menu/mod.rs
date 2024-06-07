pub mod components;
pub mod systems;

use crate::menu::systems::*;
use crate::AppState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (init, setup))
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .add_systems(
                Update,
                (
                    button_click.run_if(input_just_pressed(MouseButton::Left)),
                    button_system,
                )
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
