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

            // let offset_x = num_cols as f32 / 2. - 0.5;
            // let get_x_pos = | col: f32 | -> f32 {
            //     return (col - offset_x) * (tile_width + tile_margin);
            // };

            // let offset_y = num_rows as f32 / 2. - 0.5;
            // let get_y_pos = | row: f32 | -> f32 {
            //     return (offset_y - row) * (tile_height + tile_margin);
            // };

            let num_tiles = num_rows * num_cols;
            let get_hue = | row: f32, col: f32 | -> f32 {
                return 720. * (row + col) / num_tiles as f32;
            };

            // let meshes = world.resource_mut::<Assets<Mesh>>();
            // let materials = world.resource_mut::<Assets<ColorMaterial>>();

            world.commands().entity(entity)
                .with_children(|parent| {
                    for (col, row) in iproduct!(0..num_cols, 0..num_rows) {
                        // let color = Color::hsl(get_hue(row as f32, col as f32), 0.95, 0.7);
                        // let rect = Rectangle::new(tile_width, tile_height);
                        // let x_pos = get_x_pos(col as f32);
                        // let y_pos = get_y_pos(row as f32);

                        parent.spawn(TileBundle{
                            marker: Tile,
                            row: TileBoardRow(row),
                            col: TileBoardCol(col),
                            width: TileWidth(tile_width),
                            height: TileHeight(tile_height),
                            margin: TileMargin(tile_margin),
                            color: TileColor(Color::hsl(get_hue(row as f32, col as f32), 0.95, 0.7)),
                            sprite: MaterialMesh2dBundle::default(),
                            // sprite: MaterialMesh2dBundle {
                            //     material: materials.add(color),
                            //     mesh: Mesh2dHandle(meshes.add(rect)),
                            //     transform: Transform::from_xyz(x_pos, y_pos, 0.),
                            //     ..default()
                            // },
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

#[derive(Component)]
struct TileColor(Color);

// Bundles ---------------------------------------------------------------------
#[derive(Bundle)]
struct TileBoardBundle {
    marker: TileBoard,
    num_rows: NumTileBoardRows,
    num_cols: NumTileBoardCols,
    tile_width: TileWidth,
    tile_height: TileHeight,
    tile_margin: TileMargin,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
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
            sprite: MaterialMesh2dBundle::default(),
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
    color: TileColor,
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
        .add_systems(Update, render_tile_board)
        .run();
}

// Systems ---------------------------------------------------------------------
fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // https://bevyengine.org/news/bevy-0-14/#component-lifecycle-hooks
    commands.spawn(TileBoardBundle::new(3, 3, 100., 100., 10.).unwrap());
}

fn render_tiles(
    tiles: 
) {
    //
}
