pub mod systems;

use crate::AppState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use systems::*;

pub struct End;

impl Plugin for End {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                show_results,
                return_to_menu.run_if(input_just_pressed(MouseButton::Left)),
            )
                .run_if(in_state(AppState::End)),
        )
        .add_systems(OnExit(AppState::End), cleanup);
    }
}
