use bevy::prelude::*; // Import Bevy's core functionality

// Define a custom resource used for storing spatial data (will be expanded later)
#[derive(Resource, Default)]
struct SpatialIndex;

// Define a system that runs once at startup
fn setup() {
    todo!()
}

// Define a system that runs every frame during the app's update stage
fn draw_shapes() {
    todo!()
}

fn main() {
    App::new() // Create a new Bevy application
        .add_plugins(DefaultPlugins) // Add Bevy’s default plugins (windowing, input, rendering, etc.)
        .init_resource::<SpatialIndex>() // Initialize the SpatialIndex resource and insert it at startup
        .add_systems(Startup, setup) // Register the setup system to run during app startup
        .add_systems(Update, draw_shapes) // Register draw_shapes to run every frame during update
        .run(); // Start the app's main loop
}
