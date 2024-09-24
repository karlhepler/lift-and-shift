use bevy::prelude::*;

// Constants -------------------------------------------------------------------
const WINDOW_WIDTH: f32 = 1179./2.5; // reduced resolution for iphone 14 pro
const WINDOW_HEIGHT: f32 = 2556./2.5; // reduced resolution for iphone 14 pro

// Components ------------------------------------------------------------------
#[derive(Component)]
struct TileBoard;

#[derive(Component)]
struct NumRows(usize);

#[derive(Component)]
struct NumCols(usize);

#[derive(Component)]
struct TileWidth(f32);

#[derive(Component)]
struct TileHeight(f32);

#[derive(Component)]
struct TileMargin(f32);

// Bundles ---------------------------------------------------------------------
#[derive(Bundle)]
struct TileBoardBundle {
    marker: TileBoard,
    num_rows: NumRows,
    num_cols: NumCols,
    tile_width: TileWidth,
    tile_height: TileHeight,
    tile_margin: TileMargin,
}

impl TileBoardBundle {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        tile_width: f32,
        tile_height: f32,
        tile_margin: f32,
    ) -> Result<Self, String> {
        let max_num_rows = (WINDOW_HEIGHT / (tile_height + tile_margin)) as usize;
        if num_rows > max_num_rows {
            return Err(format!(
                "Number of rows ({}) exceeds the maximum allowed ({})",
                num_rows, max_num_rows,
            ));
        }

        let max_num_cols = (WINDOW_WIDTH / (tile_width + tile_margin)) as usize;
        if num_cols > max_num_cols {
            return Err(format!(
                "Number of columns ({}) exceeds the maximum allowed ({})",
                num_cols, max_num_cols,
            ));
        }

        Ok(Self {
            marker: TileBoard,
            num_rows: NumRows(num_rows),
            num_cols: NumCols(num_cols),
            tile_width: TileWidth(tile_width),
            tile_height: TileHeight(tile_height),
            tile_margin: TileMargin(tile_margin),
        })
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
        .run();
}

// Systems ---------------------------------------------------------------------
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // https://bevyengine.org/news/bevy-0-14/#component-lifecycle-hooks
    commands.spawn(TileBoardBundle::new(3, 3, 100., 100., 10.).unwrap());
}
