use crate::game::components::*;
use crate::game::resources::{ClearingCells, GameData, NTimer};
use crate::game::*;
use crate::{AppState, EndState, NStopWatch};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

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

pub fn show_bombs(
    mut commands: Commands,
    grid: Res<Grid>,
    cells: Query<(Entity, &Cell)>,
    game_data: Res<GameData>,
) {
    commands.insert_resource(NTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    cells.iter().for_each(|(entity, cell)| {
        if grid.is_bomb_cell(cell) {
            change_color(&mut commands, entity, game_data.bomb_color());
        }
    });
}

pub fn clear_cells(
    mut clearing_cells: ResMut<ClearingCells>,
    mut text_grid: ResMut<TextGrid>,
    mut commands: Commands,
    cells: Query<(Entity, &Cell, Option<&Flag>, Option<&Visible>)>,
    grid: Res<Grid>,
    game_data: Res<GameData>,
) {
    let mut checking_cells = Vec::with_capacity(8);
    while let Some(cell) = clearing_cells.cells.pop() {
        checking_cells.push(cell);
        if checking_cells.len() == 8 {
            break;
        }
    }

    for (entity, cell) in checking_cells {
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
            cells
                .iter()
                .filter(|(_, _, flag, visible)| flag.is_none() && visible.is_none())
                .filter(|(_, c, _, _)| cell.is_near(c) && !grid.is_bomb_cell(c))
                .for_each(|(entity, cell, _, _)| clearing_cells.cells.push((entity, cell.clone())));
        }
        change_color(&mut commands, entity, game_data.cell_color());
    }
}

pub fn check_cell(
    windows: Query<&Window>,
    cells: Query<(Entity, &Cell, Option<&Flag>)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<Grid>,
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
        let clicked = cells.iter().find(
            |(_entity, cell, _)| {
                if &&clicked_cell == cell {
                    true
                } else {
                    false
                }
            },
        );
        let center_cell =
            clicked.map(|(entity, cell, flag)| (entity, cell.clone(), flag.map(|c| c.clone())));
        if let Some((entity, cell, flag)) = center_cell {
            if grid.is_bomb_cell(&cell) && flag.is_none() {
                end_state.set(EndState::Lose);
            }
            clearing_cells.cells.push((entity, cell));
        }
    }
}

pub fn add_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), (Without<Flag>, Without<Visible>)>,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
) {
    let width = 600;
    let height = width;
    let grid_width = 20;
    let grid_height = 20;
    let cell_width = width as f32 / grid_width as f32;
    let cell_height = height as f32 / grid_height as f32;
    let mut grid = Grid::new(grid_width, grid_height, width, height);
    grid.generate(40);
    let mut game_data = GameData::default();
    game_data.setup(&mut materials, &server);
    commands.insert_resource(grid);
    commands.insert_resource(game_data);
    commands.insert_resource(TextGrid::default());
    commands.insert_resource(ClearingCells::default());
    commands.insert_resource(NStopWatch::default());
    let cell_color = materials.add(Color::rgb(1.0, 0.27, 0.0));
    let line_color = materials.add(Color::rgb(0.2, 0.2, 0.2));
    let cell_rectangle = meshes.add(Rectangle::new(cell_width, cell_height));
    let vertical_line = meshes.add(Rectangle::new(1.0, height as f32));
    let horizontal_line = meshes.add(Rectangle::new(width as f32, 1.0));
    let mut line_meshes = Vec::with_capacity((grid_width + grid_height) as usize);
    let mut cell_meshes = Vec::with_capacity((grid_width * grid_height) as usize);
    for x in 0..grid_width {
        if x > 0 {
            line_meshes.push((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(vertical_line.clone()),
                    material: line_color.clone(),
                    transform: Transform::from_xyz(x as f32 * cell_width, 300.0, 1.0),
                    ..default()
                },
                GameComponent,
            ));
            line_meshes.push((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(horizontal_line.clone()),
                    material: line_color.clone(),
                    transform: Transform::from_xyz(300.0, x as f32 * cell_height, 1.0),
                    ..default()
                },
                GameComponent,
            ));
        }
        for y in 0..grid_height {
            cell_meshes.push((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(cell_rectangle.clone()),
                    material: cell_color.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 + 0.5) * cell_width,
                        (y as f32 + 0.5) * cell_height,
                        0.0,
                    ),
                    ..default()
                },
                GameComponent,
                Cell::new(x, y),
            ));
        }
    }

    commands.spawn_batch(line_meshes);
    commands.spawn_batch(cell_meshes);
}

pub fn cleanup(entities: Query<Entity, With<GameComponent>>, mut commands: Commands) {
    entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
    commands.remove_resource::<Grid>();
    commands.remove_resource::<TextGrid>();
    commands.remove_resource::<ClearingCells>();
    commands.remove_resource::<GameData>();
    commands.remove_resource::<NTimer>();
}
