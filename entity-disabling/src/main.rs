// import bevy core
// import Disabled component.
// Disabled when added to an entity removes it from most system queries automatically (not skipped)
use bevy::{ecs::entity_disabling::Disabled, prelude::*};

fn main() {
    // point of entry
    App::new() //create a new bevy app
        .add_plugins((DefaultPlugins, MeshPickingPlugin)) //click on meshes
        .add_observer(disable_entities_on_click) // runs only when click event is detected
        .add_systems(Update, list_all_named_entities,)
        .run(); // begin game loop
}

// marker component
// it will tag the ui text entity we spawn later
// helps us find and update that text
#[derive(Component)]
struct EntityNameText;

// Define a new component, DisabledOnClick
// This is a marker component, it doesn't hold data. Mark entities we want to enable/disable
// Only entities with this component will respond to clicks to disable/enable
#[derive(Component)]
struct DisableOnClick;

// define the observer function
fn disable_entities_on_click(
    trigger: Trigger<Pointer<Click>>, // event that contains which entity was clicked
    valid_query: Query<&DisableOnClick>, // allows checking if the clicked entity has the component
    mut commands: Commands,           // lets you modify entities (insert/remove)
) {
    // get the clicked entity from the click event
    let clicked_entity = trigger.target();

    // check if entity has component
    if valid_query.contains(clicked_entity) {
        //
        commands.entity(clicked_entity).insert(Disabled);
    }
}

fn list_all_named_entities(
    query: Query<&Name>,
    mut name_text_query: Query<&mut Text, With<EntityNameText>>,
    mut commands: Commands,
) {

}