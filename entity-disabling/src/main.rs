use bevy::{ecs::entity_disabling::Disabled, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_observer(disable_entities_on_click)
        .run();
}

#[derive(Component)]
struct DisableOnClick;

fn disable_entities_on_click(
    trigger: Trigger<Pointer<Click>>,
    valid_query: Query<&DisableOnClick>,
    mut commands: Commands,
) {
    let clicked_entity = trigger.target();

    if valid_query.contains(clicked_entity) {
        commands.entity(clicked_entity).insert(Disabled);
    }
}
