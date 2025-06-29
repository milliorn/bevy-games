// import bevy core
// import Disabled component.
// Disabled when added to an entity removes it from most system queries automatically (not skipped)
use bevy::{ecs::entity_disabling::Disabled, prelude::*};

fn main() {
    // point of entry
    App::new() //create a new bevy app
        .add_plugins((DefaultPlugins, MeshPickingPlugin)) //click on meshes
        .add_observer(disable_entities_on_click) // runs only when click event is detected
        .add_systems(
            Update,
            (list_all_named_entities, reenable_entities_on_space),
        )
        .add_systems(Startup, (setup_scene, display_instructions))
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
    // Windows and text are entities and can be clicked!
    // We definitely don't want to disable the window itself,
    // because that would cause the app to close!
    if valid_query.contains(clicked_entity) {
        // Just add the `Disabled` component to the entity to disable it.
        // Note that the `Disabled` component is *only* added to the entity,
        // its children are not affected.
        commands.entity(clicked_entity).insert(Disabled);
    }
}

// define a new system function
// system runs every frame or specific schedule
// will build a list of active entity names
// The query here will not find entities with the `Disabled` component,
// because it does not explicitly include it.
fn list_all_named_entities(
    query: Query<&Name>, // query to get all entities that have a Name component. by default this will  not include disabled entities
    mut name_text_query: Query<&mut Text, With<EntityNameText>>, // looks for Text Component
    mut commands: Commands, // spawn, modify, despawn entities
) {
    // we will use this to append all entity names to this string
    let mut text_string = String::from("Named entities found:\n");

    // Query iteration order is not guaranteed, so we sort the names
    // to ensure the output is consistent.
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
    // This query can find disabled entities,
    // because it explicitly includes the `Disabled` component.
    disabled_entities: Query<Entity, With<Disabled>>, // finds all entities that have the Disabled component
    input: Res<ButtonInput<KeyCode>>,                 // let us check if key was pressed
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in disabled_entities.iter() {
            // To re-enable an entity, just remove the `Disabled` component.
            commands.entity(entity).remove::<Disabled>();
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, // get write access to mesh assets
    mut materials: ResMut<Assets<ColorMaterial>>, // get write access to color materials
) {
    // define distance apart the shapes will spread horizontally (width)
    const X_EXTENT: f32 = 900.;

    // spawn a camera so we can see
    commands.spawn(Camera2d);

    // create three different shapes
    let named_shapes = [
        (Name::new("Annulus"), meshes.add(Annulus::new(25.0, 50.0))),
        (
            Name::new("Bestagon"),
            meshes.add(RegularPolygon::new(50.0, 6)),
        ),
        (Name::new("Rhombus"), meshes.add(Rhombus::new(75.0, 100.0))),
    ];

    let num_shapes = named_shapes.len();

    //
    for (i, (name, shape)) in named_shapes.into_iter().enumerate() {
        // create unique color for each shape
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        // spawn a new entity for each shape
        commands.spawn((
            name,                                 // add Name component
            DisableOnClick,                       // add component
            Mesh2d(shape),                        // add mesh
            MeshMaterial2d(materials.add(color)), // add mesh to be rendered
            Transform::from_xyz(
                // set position
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT, // X position
                0.0,                                                            // Y position
                0.0,                                                            // Z position
            ),
        ));
    }
}

// define system function
fn display_instructions(mut commands: Commands) {
    commands.spawn((
        // create Text component
        Text::new(
            "Click an entity to disable it.\n\nPress Space to re-enable all disabled entities.",
        ),
        // create Node component to control layout and positioning on screen
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
