use crate::game::components::*;
use crate::game::resources::{ChangeCells, ClearingCells, GameData, NTimer};
use crate::game::*;
use crate::{get_path, AppState, EndState, GameSettings, NStopWatch};
use bevy::prelude::*;
use std::mem;
use std::sync::{mpsc, Arc, Mutex};

type NotModified = (Without<Flag>, Without<Visible>);

pub fn update_time(time: Res<Time>, mut stop_watch: ResMut<NStopWatch>) {
    stop_watch.0.tick(time.delta());
}

pub fn tick_timer(
    mut timer: ResMut<NTimer>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).finished() {
        app_state.set(AppState::End);
    }
}

pub fn show_bombs(mut commands: Commands, grid: Res<Grid>, mut cells: Query<(&mut Sprite, &Cell)>) {
    commands.insert_resource(NTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    cells.par_iter_mut().for_each(|(mut sprite, cell)| {
        if grid.is_bomb_cell(cell) {
            sprite.color = Color::BLACK;
        }
    });
}

pub fn change_all(
    mut change_cells: ResMut<ChangeCells>,
    mut cells: Query<(Entity, &mut Handle<Image>, &Cell)>,
    commands: Commands,
    game_data: Res<GameData>,
) {
    let change_cells = mem::take(&mut change_cells.cells);
    let commands = Arc::new(Mutex::new(commands));
    cells.par_iter_mut().for_each(|(entity, mut image, cell)| {
        if change_cells.contains(cell) {
            change_cell(image.as_mut(), game_data.open_cell());
            commands
                .clone()
                .lock()
                .unwrap()
                .entity(entity)
                .insert(Visible);
        }
    });
}

pub fn clear_cells(
    mut clearing_cells: ResMut<ClearingCells>,
    mut change_cells: ResMut<ChangeCells>,
    mut text_grid: ResMut<TextGrid>,
    mut commands: Commands,
    cells: Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>), Without<Tried>>,
    grid: Res<Grid>,
    game_data: Res<GameData>,
    game_settings: Res<GameSettings>,
    time: Res<Time>,
) {
    let mut popped = 0;
    let mut local_tried = vec![];
    let (tx, rx) = mpsc::channel();
    while let Some((entity, cell)) = clearing_cells.cells.pop_front() {
        local_tried.push(cell.clone());
        if grid.is_bomb_cell(&cell)
            || cells
                .iter()
                .find(|d| d.1 == &cell)
                .is_some_and(|d| d.2.is_some())
        {
            continue;
        }

        let bomb_cells = get_bombs(&cells, &cell, &grid);
        if bomb_cells > 0 {
            if !text_grid.contains(&cell) {
                change_cell_near_bomb(
                    &grid,
                    &mut text_grid,
                    &mut commands,
                    bomb_cells,
                    game_data.normal_text(),
                    &cell,
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
        popped += 1;
        if popped == (game_settings.speed as f32 / (time.delta_seconds() * 16.0)) as u32 {
            break;
        }
    }
    drop(tx);
    while let Ok(data) = rx.recv() {
        clearing_cells.cells.push_back(data);
    }

    clearing_cells
        .cells
        .retain(|(_, cell)| cells.iter().find(|(_, c, _, _)| &cell == c).is_some());
}

pub fn check_cell(
    windows: Query<&Window>,
    cells: Query<(Entity, &Cell, Option<&Flag>)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut grid: ResMut<Grid>,
    game_settings: Res<GameSettings>,
    mut clearing_cells: ResMut<ClearingCells>,
    mut end_state: ResMut<NextState<EndState>>,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor))
    {
        let clicked_cell = grid.global_to_grid(world_position.x, world_position.y);
        if grid.bombs().is_empty() {
            grid.generate(game_settings.bombs, Some(clicked_cell.clone()));
        }
        let clicked = cells.iter().find(|(_, cell, _)| &&clicked_cell == cell);
        let center_cell = clicked.map(|(entity, cell, flag)| (entity, cell.clone(), flag.cloned()));
        if let Some((entity, cell, flag)) = center_cell {
            if grid.is_bomb_cell(&cell) && flag.is_none() {
                end_state.set(EndState::Lose);
            }
            clearing_cells.cells.push_back((entity, cell));
        }
    }
}

pub fn add_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), NotModified>,
    grid: Res<Grid>,
    game_data: Res<GameData>,
    mut commands: Commands,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor))
    {
        let clicked_cell = grid.global_to_grid(world_position.x, world_position.y);
        if let Some((entity, cell)) = cells.iter().find(|(_, other)| &&clicked_cell == other) {
            commands.entity(entity).insert(Flag::default());
            spawn_text(
                &mut commands,
                game_data.flag_text(),
                "🚩",
                grid.grid_to_global(cell),
            )
            .insert(Flag {
                cell: Some(cell.clone()),
            })
            .insert(GameComponent);
        }
    }
}

pub fn remove_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), With<Flag>>,
    flags: Query<(Entity, &Flag), With<Text>>,
    grid: Res<Grid>,
    mut commands: Commands,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor))
    {
        let clicked_cell = grid.global_to_grid(world_position.x, world_position.y);
        if let Some((entity, cell)) = cells.iter().find(|(_, other)| &&clicked_cell == other) {
            if let Some((text, _)) = flags.iter().find(|(_, flag)| {
                flag.cell
                    .as_ref()
                    .is_some_and(|flag_cell| flag_cell == cell)
            }) {
                commands.entity(entity).remove::<Flag>();
                commands.entity(text).despawn();
            }
        }
    }
}

pub fn check_win(
    grid: Res<Grid>,
    cells: Query<&Cell>,
    visible_cells: Query<&Visible>,
    flagged: Query<&Cell, With<Flag>>,
    mut end_state: ResMut<NextState<EndState>>,
) {
    let bombs = grid.bombs();
    if cells.iter().len() - bombs.len() == visible_cells.iter().len()
        && flagged.iter().len() == bombs.len()
    {
        end_state.set(EndState::Win);
    }
}

pub fn grid_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    game_settings: Res<GameSettings>,
) {
    let grid_width = game_settings.width;
    let grid_height = game_settings.height;
    let cell_width = 30.0;
    let cell_height = 30.0;
    let width = grid_width * cell_width as u32;
    let height = grid_height * cell_height as u32;
    let grid = Grid::new(grid_width, grid_height, width, height);
    let mut game_data = GameData::default();
    game_data.setup(&server);
    commands.insert_resource(grid);
    commands.insert_resource(game_data);
    commands.insert_resource(TextGrid::default());
    commands.insert_resource(ClearingCells::default());
    commands.insert_resource(ChangeCells::default());
    commands.insert_resource(NStopWatch::default());
    let closed = server.load(get_path("textures/closed.png"));
    let mut cell_meshes = Vec::with_capacity((grid_width * grid_height) as usize);

    for x in 0..grid_width {
        for y in 0..grid_height {
            cell_meshes.push((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(cell_width, cell_height)),
                        ..default()
                    },
                    texture: closed.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 + 0.5) * cell_width,
                        (y as f32 + 0.5) * cell_height,
                        0.0,
                    ),
                    ..default()
                },
                GameComponent,
                Cell::new(x as f32, y as f32),
            ));
        }
    }

    commands.spawn_batch(cell_meshes);
}

pub fn cleanup(entities: Query<Entity, With<GameComponent>>, mut commands: Commands) {
    entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
    commands.remove_resource::<Grid>();
    commands.remove_resource::<TextGrid>();
    commands.remove_resource::<ClearingCells>();
    commands.remove_resource::<ChangeCells>();
    commands.remove_resource::<GameData>();
    commands.remove_resource::<NTimer>();
}
