use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Chess".to_string(),
                resolution: (1400., 1400.).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
