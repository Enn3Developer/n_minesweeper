use crate::game::components::*;
use crate::game::*;
use crate::{AppState, EndState};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn check_cell(
    windows: Query<&Window>,
    cells: Query<(Entity, &Cell, Option<&Flag>)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<Grid>,
    mut text_grid: ResMut<TextGrid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
    let mut style = TextStyle::default();
    style.color = Color::BLACK;
    style.font_size = 24.0;
    let color = materials.add(Color::rgb(1.0, 1.0, 1.0));
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
        let mut center_cell = clicked.map(|(entity, cell, flag)| (entity, cell, flag, true));
        let mut trying = vec![];
        let mut tried = vec![];
        while let Some((checking_entity, checking_cell, flag, check_others)) = center_cell {
            tried.push(checking_cell);
            if grid.is_bomb_cell(checking_cell) || flag.is_some() {
                println!("bomb or flag");
                center_cell = trying.pop();
                continue;
            }

            let bomb_cells = get_bombs(&cells, checking_cell, &grid);

            if bomb_cells > 0 {
                if !text_grid.contains(checking_cell) {
                    change_cell_near_bomb(
                        &grid,
                        &mut text_grid,
                        &mut commands,
                        bomb_cells,
                        style.clone(),
                        checking_cell,
                    );
                }
            } else if check_others {
                check_cells(&cells, checking_cell, &tried, &grid, &mut trying);
            }
            change_color(&mut commands, checking_entity, color.clone());
            center_cell = trying.pop();
        }
    }
}

pub fn add_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), (Without<Flag>, Without<Visible>)>,
    grid: Res<Grid>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
    let mut style = TextStyle::default();
    style.color = Color::BLACK;
    style.font_size = 24.0;
    style.font = server.load("fonts/NotoEmoji.ttf");
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor))
    {
        let clicked_cell = grid.global_to_grid(world_position.x, world_position.y);
        if let Some((entity, cell)) = cells.iter().find(|(_, other)| &&clicked_cell == other) {
            commands.entity(entity).insert(Flag::default());
            spawn_text(&mut commands, style, "🚩", grid.grid_to_global(cell))
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
    mut app_state: ResMut<NextState<AppState>>,
    mut end_state: ResMut<NextState<EndState>>,
) {
    let bombs = grid.bombs();
    if cells.iter().len() - bombs.len() == visible_cells.iter().len()
        && flagged.iter().len() == bombs.len()
    {
        app_state.set(AppState::End);
        end_state.set(EndState::Win);
    }
}

pub fn grid_setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    let width = 600;
    let height = width;
    window.resolution.set(width as f32, height as f32);
    window.resizable = false;
    window.title = String::from("N Mines");
    let grid_width = 20;
    let grid_height = 20;
    let cell_width = width as f32 / grid_width as f32;
    let cell_height = height as f32 / grid_height as f32;
    let mut grid = Grid::new(grid_width, grid_height, width, height);
    grid.generate(1);
    commands.insert_resource(grid);
    commands.insert_resource(TextGrid::default());
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(width as f32 / 2.0, height as f32 / 2.0, 0.0);
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: width as f32,
        height: height as f32,
    };
    let line_color = materials.add(Color::rgb(0.2, 0.2, 0.2));
    commands.spawn(camera);
    for x in 0..grid_width {
        for y in 0..grid_height {
            if x > 0 && y > 0 {
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, height as f32))),
                        material: line_color.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * cell_width,
                            y as f32 * cell_height,
                            1.0,
                        ),
                        ..default()
                    })
                    .insert(GameComponent);
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(width as f32, 1.0))),
                        material: line_color.clone(),
                        transform: Transform::from_xyz(
                            x as f32 * cell_width,
                            y as f32 * cell_height,
                            1.0,
                        ),
                        ..default()
                    })
                    .insert(GameComponent);
            }
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(cell_width, cell_height))),
                    material: materials.add(Color::rgb(1.0, 0.27, 0.0)),
                    transform: Transform::from_xyz(
                        (x as f32 + 0.5) * cell_width,
                        (y as f32 + 0.5) * cell_height,
                        0.0,
                    ),
                    ..default()
                })
                .insert(Cell::new(x, y))
                .insert(GameComponent);
        }
    }
}

pub fn cleanup(entities: Query<Entity, With<GameComponent>>, mut commands: Commands) {
    entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}
