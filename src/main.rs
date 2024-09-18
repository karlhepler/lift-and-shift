// Best Practices: https://github.com/tbillington/bevy_best_practices
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    input::common_conditions::*,
};
use bevy_cursor::prelude::*;

const WINDOW_WIDTH: f32 = 1179./2.5; // reduced resolution for iphone 14 pro
const WINDOW_HEIGHT: f32 = 2556./2.5; // reduced resolution for iphone 14 pro
const TILE_WIDTH: f32 = 100.;
const TILE_HEIGHT: f32 = 100.;
const TILE_MARGIN: f32 = 10.;
const MAX_NUM_COLS: usize = (WINDOW_WIDTH / (TILE_WIDTH + TILE_MARGIN)) as usize;
const MAX_NUM_ROWS: usize = (WINDOW_HEIGHT / (TILE_HEIGHT + TILE_MARGIN)) as usize;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Row(usize);

#[derive(Component)]
pub struct Col(usize);

#[derive(Bundle)]
pub struct TileBundle {
    marker: Tile,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
    row: Row,
    col: Col,
}

#[derive(Event)]
struct TileTouchedEvent(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lift and Shift".to_string(),
                // reduced resolution for iphone 14 pro
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TrackCursorPlugin)
        .add_event::<TileTouchedEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            (
                mouse_click.run_if(input_just_pressed(MouseButton::Left)),
                key_press,
            ),
            (
                despawn_tile,
                print_goodbye_tile,
                pop_sound,
            ),
        ).chain())
        .run();
}

fn print_goodbye_tile(
    mut evt_tiletouched: EventReader<TileTouchedEvent>,
) {
    for evt in evt_tiletouched.read() {
        eprintln!("Goodbye Tile: {:?}", evt.0);
    }
}

fn despawn_tile(
    mut evt_tiletouched: EventReader<TileTouchedEvent>,
    mut commands: Commands,
) {
    for evt in evt_tiletouched.read() {
        commands.entity(evt.0).despawn();
    }
}

fn pop_sound(
    mut evt_tiletouched: EventReader<TileTouchedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in evt_tiletouched.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/pop.ogg"),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
}

impl TileBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Option<Color>,
        transform: Option<Transform>,
        row: usize,
        col: usize,
    ) -> Self {
        let color = color.unwrap_or(Color::default());
        let transform = transform.unwrap_or(Transform::default());
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT)));

        Self {
            marker: Tile,
            sprite: MaterialMesh2dBundle{
                mesh,
                material: materials.add(color),
                transform,
                ..default()
            },
            row: Row(row),
            col: Col(col),
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: What about different sized tiles on the same grid?
    // TODO: What about tiles that are combined randomly... almost like tetris?
    commands.spawn(Camera2dBundle::default());

    let num_cols: usize = 3;
    let num_rows: usize = 3;
    let num_tiles = num_cols * num_rows;

    if num_cols > MAX_NUM_COLS {
        println!("too many columns");
    }
    if num_rows > MAX_NUM_ROWS {
        println!("too many rows");
    }

    let offset_x = num_cols as f32 / 2. - 0.5;
    let get_x_pos = | col: f32 | -> f32 {
        return (col - offset_x) * (TILE_WIDTH + TILE_MARGIN);
    };

    let offset_y = num_rows as f32 / 2. - 0.5;
    let get_y_pos = | row: f32 | -> f32 {
        return (offset_y - row) * (TILE_HEIGHT + TILE_MARGIN);
    };

    let get_hue = | row: f32, col: f32 | -> f32 {
        return 720. * (row + col) / num_tiles as f32;
    };

    for row in 0..num_rows {
        for col in 0..num_cols {
            let x_pos = get_x_pos(col as f32);
            let y_pos = get_y_pos(row as f32);

            commands.spawn(TileBundle::new(
                &mut meshes,
                &mut materials,
                Some(Color::hsl(get_hue(row as f32, col as f32), 0.95, 0.7)),
                Some(Transform::from_xyz(x_pos, y_pos, 0.)),
                row,
                col,
            ));
        }
    }
}

fn key_press(
    keys: Res<ButtonInput<KeyCode>>,
    mut tiles: Query<(&mut Row, &mut Col), With<Tile>>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        println!("arrow up");
        for (mut row, col) in tiles.iter_mut() {
            row.0 = row.0.saturating_sub(1);
            println!("({:?}, {:?})", col.0, row.0);
        }
        return;
    }

    if keys.just_pressed(KeyCode::ArrowRight) {
        println!("arrow right");
        for (row, mut col) in tiles.iter_mut() {
            col.0 = col.0.saturating_add(1);
            println!("({:?}, {:?})", col.0, row.0);
        }
        return;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        println!("arrow down");
        for (mut row, col) in tiles.iter_mut() {
            row.0 = row.0.saturating_add(1);
            println!("({:?}, {:?})", col.0, row.0);
        }
        return;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        println!("arrow left");
        for (row, mut col) in tiles.iter_mut() {
            col.0 = col.0.saturating_sub(1);
            println!("({:?}, {:?})", col.0, row.0);
        }
        return;
    }
}

fn mouse_click(
    cursor: Res<CursorLocation>,
    tiles: Query<(Entity, &Transform), With<Tile>>,
    mut evt_tiletouched: EventWriter<TileTouchedEvent>,
) {
    let cursor = cursor.get();
    if cursor.is_none() {
        return;
    }
    let cursor = cursor.unwrap();
    let cursor_pos = cursor.world_position;

    let half_tile_width = TILE_WIDTH/2.;
    let half_tile_height = TILE_HEIGHT/2.;

    let cursor_touching = |tile_pos: Vec3| -> bool {
        (tile_pos.x - half_tile_width) < cursor_pos.x &&
        (tile_pos.x + half_tile_width) > cursor_pos.x && 
        (tile_pos.y - half_tile_height) < cursor_pos.y &&    
        (tile_pos.y + half_tile_height) > cursor_pos.y
    };

    for (entity, transform) in tiles.iter() {
        if cursor_touching(transform.translation) {
            evt_tiletouched.send(TileTouchedEvent(entity));
            return;
        }
    }
}
