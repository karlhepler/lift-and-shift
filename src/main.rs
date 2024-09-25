use bevy::{
    prelude::*,
    ecs::component::{ComponentHooks, StorageType},
};
use itertools::iproduct;

// Constants -------------------------------------------------------------------
const WINDOW_WIDTH: f32 = 1179./2.5; // reduced resolution for iphone 14 pro
const WINDOW_HEIGHT: f32 = 2556./2.5; // reduced resolution for iphone 14 pro

// Components ------------------------------------------------------------------
#[derive(Debug)]
struct TileBoard;
impl Component for TileBoard {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            let num_rows = world.get::<NumTileBoardRows>(entity).unwrap().0;
            let num_cols = world.get::<NumTileBoardCols>(entity).unwrap().0;
            let tile_width = world.get::<TileWidth>(entity).unwrap().0;
            let tile_height = world.get::<TileHeight>(entity).unwrap().0;
            let tile_margin = world.get::<TileMargin>(entity).unwrap().0;
            world.commands().entity(entity)
                .with_children(|parent| {
                    for (col, row) in iproduct!(0..num_cols, 0..num_rows) {
                        parent.spawn(TileBundle{
                            marker: Tile,
                            row: TileBoardRow(row),
                            col: TileBoardCol(col),
                            width: TileWidth(tile_width),
                            height: TileHeight(tile_height),
                            margin: TileMargin(tile_margin),
                        });
                    }
                });
        });
    }
}

#[derive(Component)]
struct NumTileBoardRows(usize);

#[derive(Component)]
struct NumTileBoardCols(usize);

#[derive(Component)]
struct TileWidth(f32);

#[derive(Component)]
struct TileHeight(f32);

#[derive(Component)]
struct TileMargin(f32);

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TileBoardRow(usize);

#[derive(Component)]
struct TileBoardCol(usize);

// Bundles ---------------------------------------------------------------------
#[derive(Bundle)]
struct TileBoardBundle {
    marker: TileBoard,
    num_rows: NumTileBoardRows,
    num_cols: NumTileBoardCols,
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
            num_rows: NumTileBoardRows(num_rows),
            num_cols: NumTileBoardCols(num_cols),
            tile_width: TileWidth(tile_width),
            tile_height: TileHeight(tile_height),
            tile_margin: TileMargin(tile_margin),
        })
    }
}

#[derive(Bundle)]
struct TileBundle {
    marker: Tile,
    row: TileBoardRow,
    col: TileBoardCol,
    width: TileWidth,
    height: TileHeight,
    margin: TileMargin,
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
