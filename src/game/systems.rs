use crate::game::components::*;
use crate::game::resources::{ChangeCells, ClearingCells, GameData, NTimer};
use crate::game::*;
use crate::{get_path, AppState, EndState, GameSettings, NStopWatch, NTime};
use bevy::prelude::*;
use std::mem;
use std::sync::{mpsc, Arc, Mutex};
use web_time::Instant;

type NotModified = (Without<Flag>, Without<Visible>);

pub fn tick_timer(
    mut timer: ResMut<NTimer>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).finished() {
        app_state.set(AppState::End);
    }
}

pub fn show_bombs(
    mut commands: Commands,
    grid: Res<Grid>,
    cells: Query<&Cell>,
    game_data: Res<GameData>,
) {
    commands.insert_resource(NTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    let mut bombs = Vec::with_capacity(grid.bombs().len());
    for cell in cells.iter() {
        if grid.is_bomb_cell(cell) {
            bombs.push((
                prepare_spawn_sprite(
                    grid.grid_to_global(cell),
                    CellState::Bomb,
                    game_data.atlas(),
                    Color::WHITE,
                ),
                GameComponent,
            ));
        }
    }

    commands.spawn_batch(bombs);
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
    let speed = (game_settings.speed as f32 * time.delta_seconds() * 1000.0) as u32;
    let mut local_tried = vec![];
    let (tx_c, rx_c) = mpsc::channel();
    while popped < speed {
        if clearing_cells.cells.is_empty() {
            break;
        }
        let (tx, rx) = mpsc::channel();
        while let Some((entity, cell)) = clearing_cells.cells.pop_front() {
            if clear_all(
                &mut local_tried,
                cell,
                &grid,
                &cells,
                &mut text_grid,
                &mut commands,
                &game_data,
                tx.clone(),
                tx_c.clone(),
                entity,
                &mut change_cells,
            ) {
                popped += 1;
                if popped == speed {
                    break;
                }
            }
        }
        while let Ok(data) = rx.try_recv() {
            clearing_cells.cells.push_back(data);
        }
        clearing_cells
            .cells
            .retain(|(_, cell)| cells.iter().find(|(_, c, _, _)| &cell == c).is_some());
    }
    let mut sprites = vec![];
    while let Ok(sprite) = rx_c.try_recv() {
        sprites.push(sprite);
    }

    commands.spawn_batch(sprites);
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
            spawn_sprite(
                &mut commands,
                grid.grid_to_global(cell),
                CellState::Flag,
                game_data.atlas(),
                Color::BLACK,
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
    flags: Query<(Entity, &Flag), Without<Cell>>,
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

pub fn init(
    game_settings: Res<GameSettings>,
    mut q_transform: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = q_transform.single_mut();
    transform.translation = Vec3::new(
        game_settings.width as f32 * 30.0 / 2.0,
        game_settings.height as f32 * 30.0 / 2.0,
        1000.0,
    );
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
    commands.insert_resource(NStopWatch(Instant::now()));
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

pub fn cleanup(
    entities: Query<Entity, With<GameComponent>>,
    mut commands: Commands,
    stop_watch: ResMut<NStopWatch>,
) {
    entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
    commands.remove_resource::<Grid>();
    commands.remove_resource::<TextGrid>();
    commands.remove_resource::<ClearingCells>();
    commands.remove_resource::<ChangeCells>();
    commands.remove_resource::<GameData>();
    commands.remove_resource::<NTimer>();
    commands.insert_resource(NTime((Instant::now() - stop_watch.0).as_secs_f32()));
}
