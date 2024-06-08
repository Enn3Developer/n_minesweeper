pub mod components;
pub mod systems;

use crate::menu::systems::*;
use crate::AppState;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .add_systems(Update, draw_ui.run_if(in_state(AppState::MainMenu)));
    }
}
