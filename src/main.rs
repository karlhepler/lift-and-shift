use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    ecs::component::{ComponentHooks, StorageType},
};
use itertools::iproduct;

// Constants -------------------------------------------------------------------
const WINDOW_WIDTH: f32 = 1179./2.5; // reduced resolution for iphone 14 pro
const WINDOW_HEIGHT: f32 = 2556./2.5; // reduced resolution for iphone 14 pro

// Components ------------------------------------------------------------------
#[derive(Component, Debug)]
struct Board {
    cells: Vec<Vec<Cell>>,
}
#[derive(Debug)]
struct Cell {
    x: f32,
    y: f32,
    occupied_by: Option<Entity>,
}
impl Board {
    fn new(rows: usize, cols: usize, cell_size: f32) -> Self {
        let width = cols as f32 * cell_size;
        let height = rows as f32 * cell_size;
        let half_cell_size = cell_size / 2.;
        let half_width = width / 2.;
        let half_height = height / 2.;
        let offset_x = half_width - half_cell_size;
        let offset_y = half_height - half_cell_size;

        let mut cells = Vec::new();

        for row in 0..rows {
            let mut row_vec = Vec::new();
            for col in 0..cols {
                let x = col as f32 * cell_size - offset_x;
                let y = (row as f32 * cell_size - offset_y) * -1.;
                row_vec.push(Cell { x, y, occupied_by: None });
            }
            cells.push(row_vec);
        }

        Self { cells }
    }
}

#[derive(Component)]
struct Tile;

// Bundles ---------------------------------------------------------------------
#[derive(Bundle)]
struct BoardBundle {
    board: Board,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

// Main ------------------------------------------------------------------------
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lift and Shift".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, startup)
        .observe(on_add_board)
        .run();
}

// Systems ---------------------------------------------------------------------
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let rows = 3;
    let cols = 3;
    let cell_size = 110.;
    commands.spawn(BoardBundle {
        board: Board::new(rows, cols, cell_size),
        sprite: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(cols as f32 * cell_size, rows as f32 * cell_size))),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    });
}

fn on_add_board(
    trigger: Trigger<OnAdd, Board>,
    mut query: Query<&mut Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_width = 100.;
    let tile_height = 100.;

    let mut new_mesh = | width, height: f32 | -> Mesh2dHandle {
        let rect = Rectangle::new(width, height);
        let handle = meshes.add(rect);
        Mesh2dHandle(handle)
    };

    let mut new_material = | row, col: usize | -> Handle<ColorMaterial> {
        let hue = (row + col) as f32 * 50.;
        let color = Color::hsl(hue, 0.95, 0.7);
        materials.add(color)
    };

    let new_transform = | x, y: f32 | -> Transform {
        Transform::from_xyz(x, y, 1.)
    };

    if let Ok(mut board) = query.get_mut(trigger.entity()) {
        for (row, row_cells) in board.cells.iter_mut().enumerate() {
            for (col, cell) in row_cells.iter_mut().enumerate() {
                // spawn tile entity
                let tile_entity = commands.spawn(TileBundle {
                    tile: Tile,
                    sprite: MaterialMesh2dBundle {
                        mesh: new_mesh(tile_width, tile_height),
                        material: new_material(row, col),
                        transform: new_transform(cell.x, cell.y),
                        ..default()
                    },
                }).id();
                // add tile entity as child to board entity
                commands.entity(trigger.entity()).push_children(&[tile_entity]);
                // cell is occupied by the tile entity
                cell.occupied_by = Some(tile_entity);
            }
        }
    }
}
