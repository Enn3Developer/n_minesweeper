use crate::game::components::*;
use crate::game::resources::{ChangeCells, ClearingCells, GameData, NTimer};
use crate::game::*;
use crate::{get_path, AppState, EndState, GameSettings, NStopWatch};
use bevy::prelude::*;

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
    cells.iter_mut().for_each(|(mut sprite, cell)| {
        if grid.is_bomb_cell(cell) {
            sprite.color = Color::BLACK;
        }
    });
}

pub fn change_all(
    mut change_cells: ResMut<ChangeCells>,
    mut cells: Query<(Entity, &mut Handle<Image>, &Cell)>,
    mut commands: Commands,
    game_data: Res<GameData>,
) {
    while let Some(cell) = change_cells.cells.pop() {
        if let Some((entity, mut image, _)) = cells.iter_mut().find(|(_, _, c)| &&cell == c) {
            change_cell(image.as_mut(), game_data.open_cell());
            commands.entity(entity).insert(Visible);
        }
    }
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
) {
    let mut popped = 0;
    while let Some((entity, cell)) = clearing_cells.cells.pop() {
        println!("{cell:?}");
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
                .filter(|(_, c, _, _)| cell.is_near(c) && !grid.is_bomb_cell(c) && &&cell != c)
                .for_each(|(entity, cell, _, _)| clearing_cells.cells.push((entity, cell.clone())));
        }
        commands.entity(entity).insert(Tried);
        change_cells.cells.push(cell);
        popped += 1;
        if popped == game_settings.speed {
            break;
        }
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
        let clicked = cells.iter().find(|(_, cell, _)| &&clicked_cell == cell);
        let center_cell = clicked.map(|(entity, cell, flag)| (entity, cell.clone(), flag.cloned()));
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
    let mut grid = Grid::new(grid_width, grid_height, width, height);
    grid.generate(game_settings.bombs);
    let mut game_data = GameData::default();
    game_data.setup(&server);
    commands.insert_resource(grid);
    commands.insert_resource(game_data);
    commands.insert_resource(TextGrid::default());
    commands.insert_resource(ClearingCells::default());
    commands.insert_resource(ChangeCells::default());
    commands.insert_resource(NStopWatch::default());
    SpriteBundle {
        sprite: Sprite {
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
        },
        transform: Default::default(),
        global_transform: Default::default(),
        texture: Default::default(),
        visibility: Default::default(),
        inherited_visibility: Default::default(),
        view_visibility: Default::default(),
    };
    let closed = server.load(get_path("textures/closed.png"));
    let mut cell_meshes = Vec::with_capacity((grid_width * grid_height) as usize);

    for x in 0..grid_width {
        for y in 0..grid_height {
            cell_meshes.push((
                SpriteBundle {
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
