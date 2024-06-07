pub mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::*;

pub struct End;

impl Plugin for End {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::End), show_text);
    }
}
