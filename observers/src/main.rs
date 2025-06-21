use bevy::prelude::*; // Import Bevy's core functionality

fn main() {
    App::new() // Create a new Bevy application
        .add_plugins(DefaultPlugins) // Add Bevy’s default plugins (windowing, input, rendering, etc.)
        .run(); // Start the app's main loop
}
