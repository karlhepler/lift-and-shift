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
    tile: Option<Entity>,
}

impl Board {
    fn new(rows: usize, cols: usize, cell_size: f32) -> Self {
        let mut cells = Vec::new();

        for row in 0..rows {
            let mut row_vec = Vec::new();
            for col in 0..cols {
                let x = col as f32 * cell_size;
                let y = row as f32 * cell_size;

                row_vec.push(Cell { x, y, tile: None });
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
impl BoardBundle {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            sprite: MaterialMesh2dBundle::default(),
        }
    }
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl TileBundle {
    fn new() -> Self {
        //
    }
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
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // https://bevyengine.org/news/bevy-0-14/#component-lifecycle-hooks
    commands.spawn(BoardBundle::new(Board::new(3, 3, 120.)));
}

fn on_add_board(
    trigger: Trigger<OnAdd, Board>,
    mut board_query: Query<&mut Board>,
    mut commands: Commands,
) {
    if let Ok(mut board) = query.get(trigger.entity()) {
        for row in board.cells.iter_mut() {
            for col in row.iter_mut() {
                let tile_entity = commands.spawn(TileBundle {
                    tile: Tile,
                    sprite: MaterialMesh2dBundle::default(),
                }).id();
            }
        }
    }
}
