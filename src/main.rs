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
    ) -> Self {
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(100., 100.)));
        let material = materials.add(Color::linear_rgb(255., 255., 255.));

        Self {
            marker: Tile,
            sprite: MaterialMesh2dBundle{
                mesh,
                material,
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

    let padding = 10.;
    let num_tiles = 9;
    for i in 0..num_tiles {
        let row = (i / 3) as f32;
        let col = (i % 3) as f32;
        let x_pos = (col - 1.) * (100. + padding);
        let y_pos = (1. - row) * (100. + padding);

        commands
            .spawn(TileBundle::new(&mut meshes, &mut materials))
            .insert(Transform::from_xyz(x_pos, y_pos, 0.));
    }
}
