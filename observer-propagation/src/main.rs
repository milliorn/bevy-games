use bevy::{log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .run();
}
