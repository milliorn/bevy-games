use bevy::{math::primitives::Rectangle, prelude::*};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Chess".to_string(),
                resolution: (1400., 1400.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Rectangle::new(8.0, 8.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1., 0.9, 0.9),
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(-7.0, 20.0, 4.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -0.3, -0.5, -0.3),
            ..default()
        },
        ..default()
    });
}
