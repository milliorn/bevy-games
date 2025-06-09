// The `LogPlugin` allows us to print messages to the console (e.g., with `info!`, `warn!`, `error!`)
use bevy::log::LogPlugin;
// The `prelude` includes most of the commonly used Bevy types, like `App`, systems, and plugins
use bevy::prelude::*;

fn main() {
    App::new()
        // Add core Bevy systems without rendering or input (keeps it lightweight)
        .add_plugins((
            MinimalPlugins,
            // Enables logging so we can debug and trace behavior in the terminal
            LogPlugin::default(),
        ))
        // Register a setup system to run once at the beginning of the app
        .add_systems(Startup, setup)
        .run();
}

// This is a startup system that will run one time when the app starts.
// We will add logic here later to spawn entities.
fn setup() {}
