// Best Practices: https://github.com/tbillington/bevy_best_practices
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(100., 100.)));

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

    let tile_padding = 10.;
    let num_tiles = 9;
    for i in 0..num_tiles {
        let row = (i / 3) as f32;
        let col = (i % 3) as f32;
        let x_pos = (col - 1.) * (100. + tile_padding);
        let y_pos = (1. - row) * (100. + tile_padding);

        commands.spawn(TileBundle::new(
            &mut meshes,
            &mut materials,
            Some(Color::hsl(360. * i as f32 / num_tiles as f32, 0.95, 0.7)),
            Some(Transform::from_xyz(x_pos, y_pos, 0.)),
        ));
    }
}
