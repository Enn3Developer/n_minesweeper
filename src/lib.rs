use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Component, Eq, PartialEq, Debug)]
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

#[derive(Resource, Default)]
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
        Cell {
            x: (x / self.width as f32 * self.grid_width as f32).floor() as u32,
            y: (y / self.height as f32 * self.grid_height as f32).floor() as u32,
        }
    }

    pub fn grid_to_global(&self, cell: &Cell) -> (f32, f32) {
        (
            (cell.x as f32 + 0.5) * self.width as f32 / self.grid_width as f32,
            (cell.y as f32 + 0.5) * self.height as f32 / self.grid_height as f32,
        )
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
    cells: Query<(Entity, &Cell)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let window = windows.single();
    let (camera, transform) = cameras.single();
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
        if let Some((entity, cell)) = clicked {
            let mut center_cell = Some((entity, cell));
            let mut tried_all = false;
            let mut trying = vec![];
            let mut tried = vec![];
            while !tried_all {
                match center_cell {
                    None => tried_all = true,
                    Some((checking_entity, checking_cell)) => {
                        tried.push(checking_cell);
                        if grid.is_bomb_cell(checking_cell) {
                            println!("bomb");
                            center_cell = trying.pop();
                            continue;
                        }

                        let mut style = TextStyle::default();
                        style.color = Color::BLACK;
                        style.font_size = 24.0;

                        cells
                            .iter()
                            .filter(|(_, cell)| checking_cell.is_near(cell))
                            .filter(|(_, cell)| !tried.contains(cell))
                            .filter(|(entity, cell)| {
                                let bomb_cells = cells
                                    .iter()
                                    .filter(|(_, maybe_bomb)| cell.is_near(maybe_bomb))
                                    .filter(|(_, maybe_bomb)| grid.is_bomb_cell(maybe_bomb))
                                    .map(|(_, cell)| cell)
                                    .collect::<Vec<&Cell>>();
                                println!("bombs {bomb_cells:?}");
                                if !bomb_cells.is_empty() {
                                    let pos = grid.grid_to_global(cell);
                                    commands.spawn(Text2dBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                bomb_cells.len().to_string(),
                                                style.clone(),
                                            )],
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(pos.0, pos.1, 1.0),
                                        ..default()
                                    });
                                    commands
                                        .entity(*entity)
                                        .insert(materials.add(Color::rgb(1.0, 1.0, 1.0)));
                                }
                                cells.is_empty()
                            })
                            .for_each(|data| trying.push(data));
                        commands
                            .entity(checking_entity)
                            .insert(materials.add(Color::rgb(1.0, 1.0, 1.0)));
                        center_cell = trying.pop();
                    }
                }
            }
        }
    }
}
