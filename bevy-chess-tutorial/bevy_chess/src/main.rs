use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess!".into(),
                resolution: (1400.0, 1400.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_builder: bevy::render::mesh::PlaneMeshBuilder =
        Plane3d::default().mesh().size(8.0, 8.0);
    let plane_mesh: Mesh = Mesh::from(plane_builder);

    // 1) Ground plane as an 8×8 quad
    commands.spawn(PbrBundle {
        mesh: meshes.add(plane_mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.9, 0.9),
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(4.0, 0.0, 4.0)),
        ..default()
    });

    // 2) Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..default()
    });

    // 3) Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 20.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..default()
    });
}
