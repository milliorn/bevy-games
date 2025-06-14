use bevy::prelude::*;

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
    // start a minimal bevy ecs app
    App::new().run();
}

/// System: Spawns all entities and sets up the simulation.
/// Will be run at startup.
fn generate_bodies() {
    todo!()
}
