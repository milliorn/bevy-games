use bevy::prelude::*;

// stores the mass of an entity in the simulation
#[derive(Component, Default)]
struct Mass(f32);

// entry point
fn main() {
    // start a minimal bevy ecs app
    App::new().run();
}
