use crate::{Cell, Flag, Grid, TextGrid, Visible};
use bevy::asset::Handle;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{
    default, ColorMaterial, Commands, Entity, Query, Text, Text2dBundle, TextSection, TextStyle,
    Transform,
};

pub fn get_bombs(
    cells: &Query<(Entity, &Cell, Option<&Flag>)>,
    checking_cell: &Cell,
    grid: &Grid,
) -> u32 {
    cells
        .iter()
        .filter(|(_, cell, _)| checking_cell.is_near(cell))
        .filter(|(_, cell, _)| grid.is_bomb_cell(cell))
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
    );
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
    cells: &'a Query<(Entity, &Cell, Option<&Flag>)>,
    checking_cell: &Cell,
    tried: &[&Cell],
    grid: &Grid,
    trying: &mut Vec<(Entity, &'a Cell, Option<&'a Flag>, bool)>,
) {
    cells
        .iter()
        .filter(|(_, cell, _)| checking_cell.is_near(cell))
        .filter(|(_, cell, _)| !tried.contains(cell))
        .for_each(|(entity, cell, flag)| {
            trying.push((
                entity,
                cell,
                flag,
                if grid.is_bomb_cell(cell) {
                    false
                } else {
                    get_bombs(cells, cell, grid) == 0
                },
            ));
        });
}

pub fn change_color(commands: &mut Commands, entity: Entity, color: Handle<ColorMaterial>) {
    commands.entity(entity).insert((color, Visible));
}
