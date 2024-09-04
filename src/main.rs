use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // distribute colors evenly across the rainbow
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        let row = i / 3;
        let col = i % 3;

        let cell_size = 110.0;
        let x_position = (col as f32 - 1.0) * cell_size; // centered around the origin
        let y_position = (1.0 - row as f32) * cell_size; // centered around the origin

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(x_position, y_position, 0.0),
            ..default()
        });
    }
}
