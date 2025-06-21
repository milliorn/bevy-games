use bevy::prelude::*; // Import Bevy's core functionality

// Define a custom resource used for storing spatial data (will be expanded later)
#[derive(Resource, Default)]
struct SpatialIndex;

// Define a system that runs once when the app starts
fn setup() {
    todo!() // Placeholder for the setup logic
}

// Define a system that will eventually draw mines on the screen
fn draw_shapes() {
    todo!() // Will be responsible for rendering visuals
}

// Define a system that will handle user input (e.g. mouse clicks)
fn handle_click() {
    todo!() // Will detect and respond to input events
}

fn main() {
    App::new() // Create a new Bevy application
        .add_plugins(DefaultPlugins) // Add Bevy’s default plugins (windowing, input, rendering, etc.)
        .init_resource::<SpatialIndex>() // Initialize the SpatialIndex resource and insert it at startup
        .add_systems(Startup, setup) // Run the setup system once during startup
        .add_systems(Update, (draw_shapes, handle_click)) // Run both update systems every frame
        .run(); // Start the main app loop
}
