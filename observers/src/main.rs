use bevy::prelude::*; // Import Bevy's core functionality

// Define a custom resource used for storing spatial data (will be expanded later)
#[derive(Resource, Default)]
struct SpatialIndex;

// Define a custom event that will trigger explosions in a radius
#[derive(Event)]
struct ExplodeMines {
    pos: Vec2,   // Center position of the explosion
    radius: f32, // How far the explosion reaches
}

// Define an observer that reacts when ExplodeMines is triggered
fn observe_explode_mines(_trigger: Trigger<ExplodeMines>) {
    // This will later check for nearby mines and trigger explosions
    todo!()
}

fn setup() {
    todo!()
}

fn draw_shapes() {
    todo!()
}

fn handle_click() {
    todo!()
}

fn main() {
    App::new() // Create a new Bevy application
        .add_plugins(DefaultPlugins) // Add Bevy’s default plugins (windowing, input, rendering, etc.)
        .init_resource::<SpatialIndex>() // Initialize the SpatialIndex resource and insert it at startup
        .add_event::<ExplodeMines>() // Register the custom event type with the app
        .add_observer(observe_explode_mines) // Run this observer when ExplodeMines is triggered
        .add_systems(Startup, setup) // Run the setup system once during startup
        .add_systems(Update, (draw_shapes, handle_click)) // Run both update systems every frame
        .run(); // Start the main app loop
}
