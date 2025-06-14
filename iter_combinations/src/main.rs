use bevy::prelude::*;
use rand_chacha::rand_core::SeedableRng;

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
        todo!()
    }
}
