mod core;

use crate::core::{change_cell_near_bomb, change_color, check_cells, get_bombs, spawn_text};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Component, Debug, Default)]
pub struct Flag {
    cell: Option<Cell>,
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Cell {
    x: u32,
    y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn is_near(&self, other: &Self) -> bool {
        (self.x == other.x + 1 || (other.x > 0 && self.x == other.x - 1) || self.x == other.x)
            && (self.y == other.y + 1
                || (other.y > 0 && self.y == other.y - 1)
                || self.y == other.y)
    }
}

#[derive(Resource)]
pub struct Grid {
    bombs: Vec<Cell>,
    grid_width: u32,
    grid_height: u32,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(grid_width: u32, grid_height: u32, width: u32, height: u32) -> Self {
        Grid {
            bombs: vec![],
            grid_width,
            grid_height,
            width,
            height,
        }
    }

    pub fn generate(&mut self, mut bombs: u32) {
        let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
        while bombs > 0 {
            let x = rng.sample(Uniform::new(0, self.grid_width));
            let y = rng.sample(Uniform::new(0, self.grid_height));
            if self.is_bomb(x, y) {
                continue;
            }
            self.bombs.push(Cell::new(x, y));
            bombs -= 1;
        }
    }

    pub fn is_bomb(&self, x: u32, y: u32) -> bool {
        for bomb in &self.bombs {
            if bomb.x == x && bomb.y == y {
                return true;
            }
        }
        false
    }

    pub fn is_bomb_cell(&self, cell: &Cell) -> bool {
        self.is_bomb(cell.x, cell.y)
    }

    pub fn bombs(&self) -> &[Cell] {
        &self.bombs
    }

    pub fn global_to_grid(&self, x: f32, y: f32) -> Cell {
        Cell::new(
            (x / self.width as f32 * self.grid_width as f32).floor() as u32,
            (y / self.height as f32 * self.grid_height as f32).floor() as u32,
        )
    }

    pub fn grid_to_global(&self, cell: &Cell) -> (f32, f32) {
        (
            (cell.x as f32 + 0.5) * self.width as f32 / self.grid_width as f32,
            (cell.y as f32 + 0.5) * self.height as f32 / self.grid_height as f32,
        )
    }
}

#[derive(Resource, Default)]
pub struct TextGrid {
    texts: Vec<Cell>,
}

impl TextGrid {
    pub fn add(&mut self, cell: Cell) {
        self.texts.push(cell);
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.texts.contains(cell)
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
    grid.generate(40);
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
                commands.spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(1.0, height as f32))),
                    material: line_color.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * cell_width,
                        y as f32 * cell_height,
                        1.0,
                    ),
                    ..default()
                });
                commands.spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(width as f32, 1.0))),
                    material: line_color.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * cell_width,
                        y as f32 * cell_height,
                        1.0,
                    ),
                    ..default()
                });
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
                .insert(Cell::new(x, y));
        }
    }
}

pub fn check_cell(
    windows: Query<&Window>,
    cells: Query<(Entity, &Cell), Without<Flag>>,
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
            |(_entity, cell)| {
                if &&clicked_cell == cell {
                    true
                } else {
                    false
                }
            },
        );
        let mut center_cell = clicked;
        let mut trying = vec![];
        let mut tried = vec![];
        while let Some((checking_entity, checking_cell)) = center_cell {
            tried.push(checking_cell);
            if grid.is_bomb_cell(checking_cell) {
                println!("bomb");
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
            } else {
                check_cells(
                    &cells,
                    checking_cell,
                    &tried,
                    &grid,
                    &mut text_grid,
                    &mut commands,
                    style.clone(),
                    color.clone(),
                    &mut trying,
                );
            }

            change_color(&mut commands, checking_entity, color.clone());
            center_cell = trying.pop();
        }
    }
}

pub fn add_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), Without<Flag>>,
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
            commands.entity(entity).insert(Flag::default());
            spawn_text(
                &mut commands,
                TextStyle::default(),
                "⚑",
                grid.grid_to_global(cell),
            )
            .insert(Flag {
                cell: Some(cell.clone()),
            });
        }
    }
}

pub fn remove_flag(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    cells: Query<(Entity, &Cell), With<Flag>>,
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
        if let Some((entity, _)) = cells.iter().find(|(_, other)| &&clicked_cell == other) {
            commands.entity(entity).remove::<Flag>();
        }
    }
}
