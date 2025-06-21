use bevy::prelude::*; // Import Bevy's core functionality

// Define a custom resource used for storing spatial data (will be expanded later)
#[derive(Resource, Default)]
struct SpatialIndex;

fn main() {
    App::new() // Create a new Bevy application
        .add_plugins(DefaultPlugins) // Add Bevy’s default plugins (windowing, input, rendering, etc.)
        .init_resource::<SpatialIndex>() // Initialize the SpatialIndex resource and insert it at startup
        .run(); // Start the app's main loop
}
