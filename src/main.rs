// Best Practices: https://github.com/tbillington/bevy_best_practices
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
    input::common_conditions::*,
};

const TILE_WIDTH: f32 = 100.;
const TILE_HEIGHT: f32 = 100.;
const TILE_MARGIN: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lift and Shift".to_string(),
                // reduced resolution for iphone 14 pro
                resolution: (1179./2.5, 2556./2.5).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click.run_if(input_just_pressed(MouseButton::Left)))
        .run();
}

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    marker: Tile,
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl TileBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Option<Color>,
        transform: Option<Transform>,
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
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let num_cols = 3;
    let num_rows = 3;
    let num_tiles = num_cols + num_rows;

    let offset_x = num_cols as f32 / 2. - 0.5;
    let offset_y = num_rows as f32 / 2. - 0.5;

    let get_x_pos = | col: f32 | -> f32 {
        return (col - offset_x) * (TILE_WIDTH + TILE_MARGIN);
    };
    let get_y_pos = | row: f32 | -> f32 {
        return (offset_y - row) * (TILE_HEIGHT + TILE_MARGIN);
    };
    let get_hue = | row: f32, col: f32 | -> f32 {
        return 360. * (row + col) / num_tiles as f32;
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
            ));
        }
    }
}

fn mouse_click(
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let position = windows.single().cursor_position();
    if position.is_none() {
        return;
    }
    let position = position.unwrap();

    println!("Cursor Position: {:?}", position);
}
