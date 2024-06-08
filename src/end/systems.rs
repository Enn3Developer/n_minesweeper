use crate::{AppState, EndState, NStopWatch};
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::TextStyle::{Body, Heading, Monospace, Small};
use bevy_egui::egui::{emath, FontId};
use bevy_egui::{egui, EguiContexts};

pub fn show_results(
    end_state: Res<State<EndState>>,
    stop_watch: Res<NStopWatch>,
    mut ctx: EguiContexts,
) {
    let text = match end_state.get() {
        EndState::NotEnded => unreachable!(),
        EndState::Win => "You won!",
        EndState::Lose => "You lose!",
    };

    let ctx = ctx.ctx_mut();
    ctx.style_mut(|style| {
        style.text_styles = [
            (Heading, FontId::new(32.0, Proportional)),
            (Body, FontId::new(26.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (egui::TextStyle::Button, FontId::new(22.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.allocate_space(emath::Vec2::new(1.0, 100.0));
            ui.heading(text);
            ui.heading(format!("{:0.2} seconds", stop_watch.0.elapsed_secs()));
            ui.label("Click to return to the main menu");
        });
    });
}

pub fn return_to_menu(
    mut app_state: ResMut<NextState<AppState>>,
    mut end_state: ResMut<NextState<EndState>>,
) {
    app_state.set(AppState::MainMenu);
    end_state.set(EndState::NotEnded);
}

pub fn cleanup(texts: Query<Entity, With<Text>>, mut commands: Commands) {
    texts
        .iter()
        .for_each(|text| commands.entity(text).despawn());
}
