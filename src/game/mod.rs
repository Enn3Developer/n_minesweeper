pub mod components;
pub mod resources;
pub mod systems;

use crate::game::components::*;
use crate::game::resources::{Grid, TextGrid};
use crate::AppState;
use bevy::ecs::system::EntityCommands;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use systems::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), grid_setup)
            .add_systems(
                Update,
                (
                    check_win,
                    clear_cells,
                    (
                        check_cell.run_if(input_just_pressed(MouseButton::Left)),
                        (add_flag, remove_flag)
                            .run_if(input_just_pressed(MouseButton::Right))
                            .before(check_cell),
                    )
                        .before(check_win),
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(AppState::Playing), cleanup);
    }
}

pub fn get_bombs(
    cells: &Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>)>,
    checking_cell: &Cell,
    grid: &Grid,
) -> u32 {
    cells
        .iter()
        .filter(|(_, cell, _, _)| checking_cell.is_near(cell))
        .filter(|(_, cell, _, _)| grid.is_bomb_cell(cell))
        .count() as u32
}

pub fn change_cell_near_bomb(
    grid: &Grid,
    text_grid: &mut TextGrid,
    commands: &mut Commands,
    bomb_cells: u32,
    style: TextStyle,
    cell: &Cell,
) {
    spawn_text(
        commands,
        style,
        bomb_cells.to_string(),
        grid.grid_to_global(cell),
    )
    .insert(GameComponent);
    text_grid.add(cell.clone());
}

pub fn spawn_text<'a>(
    commands: &'a mut Commands,
    style: TextStyle,
    text: impl Into<String>,
    pos: (f32, f32),
) -> EntityCommands<'a> {
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(text, style)],
            ..default()
        },
        transform: Transform::from_xyz(pos.0, pos.1, 1.0),
        ..default()
    })
}

pub fn check_cells<'a>(
    cells: &'a Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>)>,
    checking_cell: &Cell,
    tried: &[&Cell],
    grid: &Grid,
    trying: &mut Vec<(Entity, &'a Cell, Option<&'a Flag>, bool)>,
) {
    cells
        .iter()
        .filter(|(_, cell, _, _)| checking_cell.is_near(cell))
        .filter(|(_, cell, _, _)| !tried.contains(cell))
        .for_each(|(entity, cell, flag, _)| {
            trying.push((entity, cell, flag, get_bombs(cells, cell, grid) == 0));
        });
}

pub fn change_color(commands: &mut Commands, entity: Entity, color: Handle<ColorMaterial>) {
    commands.entity(entity).insert((color, Visible));
}
