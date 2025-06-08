// import bevy core
// import Disabled component.
// Disabled when added to an entity removes it from most system queries automatically (not skipped)
use bevy::{ecs::entity_disabling::Disabled, prelude::*};

fn main() {
    // point of entry
    App::new() //create a new bevy app
        .add_plugins((DefaultPlugins, MeshPickingPlugin)) //click on meshes
        .add_observer(disable_entities_on_click) // runs only when click event is detected
        .add_systems(Update, (list_all_named_entities, reenable_entities_on_space))
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

// define a new system function
// system runs every frame or specific schedule
// will build a list of active entity names
fn list_all_named_entities(
    query: Query<&Name>, // query to get all entities that have a Name component. by default this will  not include disabled entities
    mut name_text_query: Query<&mut Text, With<EntityNameText>>, // looks for Text Component
    mut commands: Commands, // spawn, modify, despawn entities
) {
    // we will use this to append all entity names to this string
    let mut text_string = String::from("Named entities found:\n");

    // iterate all names returned by query and sort because if we dont the order might change on every frame (no certain stable order)
    for name in query.iter().sort::<&Name>() {
        // add name to text_string
        text_string.push_str(&format!("{:?}\n", name));
    }

    // Try to get the one Text entity tagged with EntityNameText.
    if let Ok(mut text) = name_text_query.single_mut() {
        // overwrite Text with the new text_string
        *text = Text::new(text_string);
    } else {
        commands.spawn((
            EntityNameText,
            Text::default(),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                ..default()
            },
        ));
    }
}

// if key is pressed it loops over all disabled entities and removed Disabled, re-enabling them
fn reenable_entities_on_space(
    mut commands: Commands,
    disabled_entities: Query<Entity, With<Disabled>>, // finds all entities that have the Disabled component
    input: Res<ButtonInput<KeyCode>>,                 // let us check if key was pressed
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in disabled_entities.iter() {
            commands.entity(entity).remove::<Disabled>();
        }
    }
}
