pub mod systems;

use crate::menu::systems::*;
use crate::{AppState, GameSettings};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::{emath, Response, Ui, Widget, WidgetText};
use bevy_egui::{egui, EguiPlugin};

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(GameSettings::default())
            .init_state::<MenuState>()
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .add_systems(Update, draw_ui.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    None,
    Customizing,
    Settings,
    Multiplayer,
    MultiplayerJoin,
}

pub fn control_buttons(
    ui: &mut Ui,
    app_state: &mut NextState<AppState>,
    next_state: &mut NextState<MenuState>,
    app_exit_events: &mut EventWriter<AppExit>,
) {
    ui.vertical_centered(|ui| {
        ui.allocate_space(emath::Vec2::new(1.0, 100.0));
        if button(ui, "Play").clicked() {
            app_state.set(AppState::Playing);
            next_state.set(MenuState::None);
        }
        if button(ui, "Customize").clicked() {
            next_state.set(MenuState::Customizing);
        }
        if button(ui, "Settings").clicked() {
            next_state.set(MenuState::Settings);
        }
        if button(ui, "Multiplayer").clicked() {
            next_state.set(MenuState::Multiplayer);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if button(ui, "Exit").clicked() {
            app_exit_events.send(AppExit::Success);
        }
    });
}

pub fn button(ui: &mut Ui, text: impl Into<WidgetText>) -> Response {
    egui::Button::new(text)
        .min_size(egui::Vec2::new(150.0, 25.0))
        .ui(ui)
}
