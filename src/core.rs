use crate::{Cell, Flag, Grid, TextGrid};
use bevy::asset::Handle;
use bevy::prelude::{
    default, ColorMaterial, Commands, Entity, Query, Text, Text2dBundle, TextSection, TextStyle,
    Transform, Without,
};

pub fn get_bombs(
    cells: &Query<(Entity, &Cell), Without<Flag>>,
    checking_cell: &Cell,
    grid: &Grid,
) -> u32 {
    cells
        .iter()
        .filter(|(_, cell)| checking_cell.is_near(cell))
        .filter(|(_, cell)| grid.is_bomb_cell(cell))
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
    let pos = grid.grid_to_global(cell);
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(bomb_cells.to_string(), style)],
            ..default()
        },
        transform: Transform::from_xyz(pos.0, pos.1, 1.0),
        ..default()
    });
    text_grid.add(cell.clone());
}

pub fn check_cells<'a>(
    cells: &'a Query<(Entity, &Cell), Without<Flag>>,
    checking_cell: &Cell,
    tried: &[&Cell],
    grid: &Grid,
    text_grid: &mut TextGrid,
    mut commands: &mut Commands,
    style: TextStyle,
    color: Handle<ColorMaterial>,
    trying: &mut Vec<(Entity, &'a Cell)>,
) {
    cells
        .iter()
        .filter(|(_, cell)| checking_cell.is_near(cell))
        .filter(|(_, cell)| !tried.contains(cell))
        .filter(|(entity, cell)| {
            if grid.is_bomb_cell(cell) {
                false
            } else {
                let bomb_cells = get_bombs(cells, cell, grid);
                if bomb_cells > 0 && !text_grid.contains(cell) {
                    change_cell_near_bomb(
                        grid,
                        text_grid,
                        commands,
                        bomb_cells,
                        style.clone(),
                        cell,
                    );
                    change_color(&mut commands, *entity, color.clone());
                }
                bomb_cells == 0
            }
        })
        .for_each(|data| trying.push(data));
}

pub fn change_color(commands: &mut Commands, entity: Entity, color: Handle<ColorMaterial>) {
    commands.entity(entity).insert(color);
}
