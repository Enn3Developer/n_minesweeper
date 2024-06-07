use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use n_minesweeper::{add_flag, check_cell, check_win, grid_setup, remove_flag};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, grid_setup)
        .add_systems(
            Update,
            (
                check_win,
                (
                    check_cell.run_if(input_just_pressed(MouseButton::Left)),
                    (add_flag, remove_flag).run_if(input_just_pressed(MouseButton::Right)),
                )
                    .before(check_win),
            ),
        );
    app.run();
}
