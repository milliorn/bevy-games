use bevy::prelude::*; // Import Bevy's core functionality
use rand::{Rng, SeedableRng}; // Import traits for random number generation
use rand_chacha::ChaCha8Rng; // Import the ChaCha8 random number generator

// Define a custom resource used for storing spatial data (will be expanded later)
#[derive(Resource, Default)]
struct SpatialIndex;

#[derive(Event)]
struct Explode;

// Define a custom event that will trigger explosions in a radius
#[derive(Event)]
struct ExplodeMines {
    pos: Vec2,   // Center position of the explosion
    radius: f32, // How far the explosion reaches
}

// Define a component representing a mine in the world
#[derive(Component)]
struct Mine {
    pos: Vec2, // The 2D position of the mine
    size: f32, // The radius of the mine
}

impl Mine {
    fn random(rand: &mut ChaCha8Rng) -> Self {
        Mine {
            pos: Vec2::new(
                (rand.random::<f32>() - 0.5) * 1200.0,
                (rand.random::<f32>() - 0.5) * 600.0,
            ),
            size: 4.0 + rand.random::<f32>() * 16.0,
        }
    }
}

fn observe_explode_mines(_trigger: Trigger<ExplodeMines>) {
    // Will react to ExplodeMines events
    todo!()
}

fn explode_mine(trigger: Trigger<Explode>, query: Query<&Mine>, mut commands: Commands) {
    // If a triggered event is targeting a specific entity you can access it with `.target()`
    let id = trigger.target();

    // If the entity exists, get a handle to it
    let Ok(mut entity) = commands.get_entity(id) else {
        return;
    };

    // Print a message to the console for debugging
    info!("Boom! {} exploded.", id.index());

    // Despawn the mine entity
    entity.despawn();

    // Read the mine's position and size from the query
    let mine = query.get(id).unwrap();

    // Trigger another explosion cascade using that position and size
    commands.trigger(ExplodeMines {
        pos: mine.pos,
        radius: mine.size,
    });
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera so we can see the scene
    commands.spawn(Camera2d);

    // Spawn on-screen instructions using a Text node positioned in the top-left corner
    commands.spawn((
        Text::new(
            "Click on a \"Mine\" to trigger it.\n\
            When it explodes it will trigger all overlapping mines.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    commands
        .spawn(Mine::random(&mut rng))
        // Observers can watch for events targeting a specific entity.
        // This will create a new observer that runs whenever the Explode event
        // is triggered for this spawned entity.
        .observe(explode_mine);

    // We want to spawn a bunch of mines. We could just call the code above for each of them.
    // That would create a new observer instance for every Mine entity. Having duplicate observers
    // generally isn't worth worrying about as the overhead is low. But if you want to be maximally efficient,
    // you can reuse observers across entities.
    //
    // First, observers are actually just entities with the Observer component! The `observe()` functions
    // you've seen so far in this example are just shorthand for manually spawning an observer.
    let mut observer = Observer::new(explode_mine);

    for _ in 0..1000 {
        let entity = commands.spawn(Mine::random(&mut rng)).id();
        observer.watch_entity(entity);
    }

    commands.spawn(observer);
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
