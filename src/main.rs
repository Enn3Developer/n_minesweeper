use bevy::prelude::*;
use n_minesweeper::{AppState, EndState, NMines};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NMines)
        .init_state::<AppState>()
        .init_state::<EndState>();
    app.run();
}
