use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Chess!".into(),
                resolution: (1400.0, 1400.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_board)
        .add_systems(Startup, create_pieces)
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

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Build a 1×1 Plane mesh
    let square_mesh: Handle<Mesh> = {
        let builder = Plane3d::default().mesh().size(1.0, 1.0);
        meshes.add(Mesh::from(builder))
    };

    // Two materials for light/dark squares
    let light_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.9),
        ..default()
    });
    let dark_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.1, 0.1),
        ..default()
    });

    // Spawn an 8×8 grid of squares, each lifted by y=0.01 and shifted by +0.5 in X/Z
    for i in 0..8 {
        for j in 0..8 {
            let use_light = (i + j + 1) % 2 == 0;
            commands.spawn(PbrBundle {
                mesh: square_mesh.clone(),

                material: if use_light {
                    light_material.clone()
                } else {
                    dark_material.clone()
                },

                transform: Transform::from_translation(Vec3::new(
                    i as f32 + 0.5,
                    0.01,
                    j as f32 + 0.5,
                )),

                ..default()
            });
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes:
    let king_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh1/Primitive0");

    // Add two materials (light and dark):
    let light_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.8),
        ..default()
    });

    // let dark_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
    //     base_color: Color::srgb(0.0, 0.2, 0.2),
    //     ..default()
    // });

    // Spawn a “parent” entity using SpatialBundle
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: king_handle.clone(),
                material: light_material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0.0, -1.9),
                    scale: Vec3::splat(0.2),
                    ..default()
                },
                ..default()
            });

            parent.spawn(PbrBundle {
                mesh: king_cross_handle.clone(),
                material: light_material.clone(),
                transform: Transform {
                    translation: Vec3::new(-0.2, 0.0, -1.9),
                    scale: Vec3::splat(0.2),
                    ..default()
                },
                ..default()
            });
        });
}
