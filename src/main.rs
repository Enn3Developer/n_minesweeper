use bevy::prelude::*;
use n_minesweeper::{embedded_asset, AppState, EndState, NMines};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(NMines)
        .init_state::<AppState>()
        .init_state::<EndState>();
    embedded_asset!(
        app,
        "src",
        "../assets/fonts/NotoEmoji.ttf",
        "fonts/NotoEmoji.ttf"
    );
    app.run();
}
