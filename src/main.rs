use bevy::asset::embedded_asset;
use bevy::prelude::*;
use n_minesweeper::{AppState, EndState, NMines};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NMines)
        .init_state::<AppState>()
        .init_state::<EndState>();
    embedded_asset!(app, "../assets/fonts/NotoEmoji.ttf");
    embedded_asset!(app, "../assets/textures/closed.png");
    embedded_asset!(app, "../assets/textures/open.png");
    app.run();
}
