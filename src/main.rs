use bevy::prelude::*;
use n_minesweeper::{AppState, EndState, NMines};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NMines)
        .insert_state(AppState::Playing)
        .init_state::<EndState>();
    app.run();
}
