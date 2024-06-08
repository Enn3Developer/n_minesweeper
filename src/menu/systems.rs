use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::TextStyle::{Body, Heading, Monospace, Small};
use bevy_egui::egui::{emath, FontId};
use bevy_egui::{egui, EguiContexts};

pub fn despawn_ui<T: Component>(
    mut commands: Commands,
    query: Query<Entity, (With<T>, Without<Camera>)>,
) {
    query
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn draw_ui(
    mut ctx: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let ctx = ctx.ctx_mut();
    ctx.style_mut(|style| {
        style.text_styles = [
            (Heading, FontId::new(40.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (egui::TextStyle::Button, FontId::new(22.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
    });

    egui::TopBottomPanel::top("title").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("N Mines");
            ui.horizontal_centered(|ui| {
                ui.add_space(1.0);
                ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
            });
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.vertical_centered(|ui| {
                ui.allocate_space(emath::Vec2::new(1.0, 100.0));
                if ui.button("Play").clicked() {
                    app_state.set(AppState::Playing);
                }
                if ui.button("Exit").clicked() {
                    app_exit_events.send(AppExit);
                }
            });
        });
    });
}

pub fn cleanup(nodes: Query<Entity, With<Node>>, mut commands: Commands) {
    nodes
        .iter()
        .for_each(|node| commands.entity(node).despawn());
}
