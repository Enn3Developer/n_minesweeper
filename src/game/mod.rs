pub mod components;
pub mod resources;
pub mod systems;

use crate::game::components::*;
use crate::game::resources::{ChangeCells, GameData, Grid, TextGrid};
use crate::{AppState, EndState};
use bevy::ecs::system::EntityCommands;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::sync::mpsc::Sender;
use systems::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), (grid_setup, init))
            .add_systems(
                Update,
                (
                    check_win,
                    clear_cells,
                    (
                        check_cell.run_if(input_just_pressed(MouseButton::Left)),
                        (add_flag, remove_flag)
                            .run_if(input_just_pressed(MouseButton::Right))
                            .before(check_cell)
                            .before(clear_cells),
                    )
                        .before(check_win)
                        .run_if(in_state(EndState::NotEnded)),
                    change_all,
                    tick_timer.run_if(not(in_state(EndState::NotEnded))),
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(EndState::NotEnded), show_bombs)
            .add_systems(OnExit(AppState::Playing), cleanup);
    }
}

pub enum CellState {
    Close,
    Near(u32),
    Flag,
    Bomb,
}

pub fn get_bombs(
    cells: &Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>), Without<Tried>>,
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
    cell: &Cell,
    atlas: Handle<Image>,
) {
    spawn_sprite(
        commands,
        grid.grid_to_global(cell),
        CellState::Near(bomb_cells),
        atlas,
        Color::BLACK,
    )
    .insert(GameComponent);
    text_grid.add(cell.clone());
}

pub fn spawn_sprite<'a>(
    commands: &'a mut Commands,
    pos: (f32, f32),
    state: CellState,
    atlas: Handle<Image>,
    color: Color,
) -> EntityCommands<'a> {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30.0, 30.0)),
            rect: Some(match state {
                CellState::Close => {
                    unreachable!()
                }
                CellState::Near(value) => Rect::from_corners(
                    Vec2::new(0.0, 2048.0 + (1024 * (value - 1)) as f32),
                    Vec2::new(1024.0, 2048.0 + (1024 * (value - 1)) as f32 + 1024.0),
                ),
                CellState::Flag => {
                    Rect::from_corners(Vec2::new(0.0, 0.0), Vec2::new(1024.0, 1024.0))
                }
                CellState::Bomb => {
                    Rect::from_corners(Vec2::new(0.0, 1024.0), Vec2::new(1024.0, 2048.0))
                }
            }),
            color,
            ..default()
        },
        texture: atlas,
        transform: Transform::from_xyz(pos.0, pos.1, 10.0),
        ..default()
    })
}

pub fn change_color(commands: &mut Commands, entity: Entity, color: Handle<ColorMaterial>) {
    commands.entity(entity).insert((color, Visible));
}

pub fn change_cell(cell_image: &mut Handle<Image>, image: Handle<Image>) {
    *cell_image = image;
}

pub fn clear_all(
    local_tried: &mut Vec<Cell>,
    cell: Cell,
    grid: &Grid,
    cells: &Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>), Without<Tried>>,
    text_grid: &mut TextGrid,
    mut commands: &mut Commands,
    game_data: &GameData,
    tx: Sender<(Entity, Cell)>,
    entity: Entity,
    change_cells: &mut ChangeCells,
) -> bool {
    local_tried.push(cell.clone());
    if grid.is_bomb_cell(&cell)
        || cells
            .iter()
            .find(|d| d.1 == &cell)
            .is_some_and(|d| d.2.is_some())
    {
        return false;
    }

    let bomb_cells = get_bombs(&cells, &cell, &grid);
    if bomb_cells > 0 {
        if !text_grid.contains(&cell) {
            change_cell_near_bomb(
                &grid,
                text_grid,
                &mut commands,
                bomb_cells,
                &cell,
                game_data.atlas(),
            );
        }
    } else {
        let tx = tx.clone();
        cells.par_iter().for_each(|(entity, c, flag, visible)| {
            if flag.is_none()
                && visible.is_none()
                && cell.is_near(c)
                && !grid.is_bomb_cell(c)
                && &cell != c
                && !local_tried.contains(c)
            {
                tx.send((entity, c.clone())).expect("can't send cell data");
            }
        });
    }
    commands.entity(entity).insert(Tried);
    change_cells.cells.push(cell);
    true
}
