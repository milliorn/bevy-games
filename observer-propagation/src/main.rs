// The `LogPlugin` allows us to print messages to the console (e.g., with `info!`, `warn!`, `error!`)
use bevy::log::LogPlugin;
// The `prelude` includes most of the commonly used Bevy types, like `App`, systems, components, and plugins
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

// The setup system runs once when the app starts.
// We spawn a Goblin with a Name and attach a Helmet as its child.
fn setup(mut commands: Commands) {
    commands
        .spawn(Name::new("Goblin")) // Parent entity
        .with_children(|parent| {
            // Child entities
            parent.spawn(Name::new("Helmet"));
            parent.spawn(Name::new("Shirt"));
            parent.spawn(Name::new("Socks"));
        });
}
