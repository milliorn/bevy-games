use bevy::{color::palettes::css::ORANGE_RED, math::FloatPow, prelude::*};
use rand::{Rng, SeedableRng};

// Gravitational constant for the simulation's physics.
const GRAVITY_CONSTANT: f32 = 0.001;

// Number of bodies (planets) to simulate.
const NUM_BODIES: usize = 100;

// stores the mass of an entity in the simulation
#[derive(Component, Default)]
struct Mass(f32);

// stores the current acceleration of an entity
#[derive(Component, Default)]
struct Acceleration(Vec3);

// stores the last position of an entity for physics calculations.
#[derive(Component, Default)]
struct LastPos(Vec3);

// marks an entity as the central star
#[derive(Component)]
struct Star;

// groups common components for a body entity
// bundle is used to group multiple components together
#[derive(Bundle, Default)]
struct BodyBundle {
    mesh: Mesh3d, // tells bevy that this entity should be rendered as a 3d mesh
    material: MeshMaterial3d<StandardMaterial>, // tells bevy how the 3d mesh should look
    mass: Mass,
    last_pos: LastPos,
    acceleration: Acceleration,
}

// entry point
fn main() {
    App::new().add_systems(Startup, generate_bodies).run();
}

/// System: Spawns all entities and sets up the simulation.
/// Will be run at startup.
fn generate_bodies(
    // Gives the current time, including timestep size.
    time: Res<Time<Fixed>>,
    // Used to spawn entities into the world.
    mut commands: Commands,
    // Used to store and reuse 3D meshes.
    mut meshes: ResMut<Assets<Mesh>>,
    // Used to store and reuse materials (appearance).
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a reusable sphere mesh for bodies.
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());

    // Range for randomizing body color (between 0.5 and 1.0 for each channel).
    let color_range = 0.5..1.0;
    // Range for randomizing initial velocity (-0.5 to 0.5).
    let vel_range = -0.5..0.5;

    // Seed the random number generator to make the simulation deterministic.
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(19878367467713);

    // Spawn multiple bodies (planets) with randomized properties.
    for _ in 0..NUM_BODIES {
        // Randomized radius for this body.
        let radius: f32 = rng.random_range(0.1..0.7);

        // Calculate the mass of the body based on its radius.
        let mass_value = FloatPow::cubed(radius) * 10.;

        // Randomize the initial position in 3D space.
        let position = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize()
            * ops::cbrt(rng.random_range(0.2f32..1.0))
            * 15.;

        // spawn a new entity representing a body in the simulation
        commands.spawn((
            // group all components for a body
            BodyBundle {
                // 3d shape of the body (sphere)
                mesh: Mesh3d(mesh.clone()),
                // random color material for appearance
                material: MeshMaterial3d(materials.add(Color::srgb(
                    rng.random_range(color_range.clone()), // red
                    rng.random_range(color_range.clone()), // green
                    rng.random_range(color_range.clone()), // blue
                ))),
                // physical mass calculated from radius
                mass: Mass(mass_value),
                // initial acceleration
                acceleration: Acceleration(Vec3::ZERO),
                // previous position for verlet integration
                last_pos: LastPos(
                    position
                        - Vec3::new(
                            rng.random_range(vel_range.clone()), // velocity x
                            rng.random_range(vel_range.clone()), // velocity y
                            rng.random_range(vel_range.clone()), // velocity z
                        ) * time.timestep().as_secs_f32(),
                ),
            },
            // position and scale in 3d space
            Transform {
                translation: position,
                scale: Vec3::splat(radius),
                ..default()
            },
        ));
    }

    // add bigger "star" body in the center
    let star_radius = 1.;
    commands
        .spawn((
            // group all components for the star
            BodyBundle {
                mesh: Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
                material: MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: ORANGE_RED.into(),
                    emissive: LinearRgba::from(ORANGE_RED) * 2.,
                    ..default()
                })),

                mass: Mass(500.0),
                ..default()
            },
            // star's scale (size)
            Transform::from_scale(Vec3::splat(star_radius)),
            Star,
        ))
        // add a child PointLight to make the star glow
        .with_child(PointLight {
            color: Color::WHITE,
            range: 100.0,
            radius: star_radius,
            ..default()
        });
}
