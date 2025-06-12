use std::time::Duration;

// The `LogPlugin` allows us to print messages to the console (e.g., with `info!`, `warn!`, `error!`)
use bevy::log::LogPlugin;
// The `prelude` includes most of the commonly used Bevy types, like `App`, systems, components, and plugins
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::seq::IteratorRandom;
use rand::{Rng, rng};

// Custom component representing the strength of an armor piece.
#[derive(Component)]
struct Armor(u16);

// this event represents an attack that deals damage to an entity
// It is configured to propagate from child to parent (e.g., from armor to Goblin).
#[derive(Clone, Component, Event)]
#[event(traversal = &'static ChildOf, auto_propagate)]
struct Attack {
    damage: u16,
}

// custom component representing health
// we use Deref to access the inner value
#[derive(Component, Deref, DerefMut)]
struct HitPoints(u16);

fn main() {
    App::new()
        // Add core Bevy systems without rendering or input (keeps it lightweight)
        .add_plugins((
            MinimalPlugins,
            // Enables logging so we can debug and trace behavior in the terminal
            LogPlugin::default(),
        ))
        // register the attack event
        .add_event::<Attack>()
        // Register a setup system to run once at the beginning of the app
        .add_systems(Startup, setup)
        // trigger attacks on armor
        .add_systems(
            Update,
            attack_armor.run_if(on_timer(Duration::from_millis(1000))),
        )
        .add_observer(attack_hits)
        .run();
}

// The setup system runs once when the app starts.
// We spawn a Goblin with a Name and attach a Helmet, Shirt, Socks as its child/ren.
fn setup(mut commands: Commands) {
    commands
        .spawn(Name::new("Goblin")) // Parent entity
        .observe(take_damage)
        .with_children(|parent| {
            // Child entities
            parent
                .spawn((Name::new("Helmet"), Armor(5)))
                .observe(block_attack);
            parent
                .spawn((Name::new("Shirt"), Armor(15)))
                .observe(block_attack);
            parent
                .spawn((Name::new("Socks"), Armor(10)))
                .observe(block_attack);
        });
}

// This observer system is called on each entity with Armor when an Attack reaches it.
// It checks if the armor blocks the damage. If it does, the event stops.
fn block_attack(mut trigger: Trigger<Attack>, armor: Query<(&Armor, &Name)>) {
    let (armor, name) = armor.get(trigger.target()).unwrap();
    let attack = trigger.event_mut();

    // calculate leftover damage after armor blocks damage
    let damage = attack.damage.saturating_sub(armor.0);

    if damage > 0 {
        info!("🩸 {} damage passed through {}", damage, name);
        // update damage
        attack.damage = damage;
    } else {
        info!("🛡️  {} damage blocked by {}", attack.damage, name);
        trigger.propagate(false);
        info!("(propagation halted early)\n");
    }
}

// take damage if armor fails to block it
fn take_damage(
    trigger: Trigger<Attack>,
    mut hp: Query<(&mut HitPoints, &Name)>,
    mut commands: Commands,
    mut app_exit: EventWriter<AppExit>,
) {
    let attack = trigger.event();
    let (mut hp, name) = hp.get_mut(trigger.target()).unwrap();

    **hp = hp.saturating_sub(attack.damage);

    if **hp > 0 {
        info!("{} has {:.1} HP", name, hp.0);
    } else {
        warn!("💀 {} has died a gruesome death", name);
        commands.entity(trigger.target()).despawn();
        app_exit.write(AppExit::Success);
    }

    info!("(propagation reached root)\n");
}

// System that triggers random attacks on one piece of armor
fn attack_armor(entities: Query<Entity, With<Armor>>, mut commands: Commands) {
    let mut rng = rng();

    if let Some(target) = entities.iter().choose(&mut rng) {
        let damage = rng.random_range(1..20);

        commands.trigger_targets(Attack { damage }, target);

        info!("⚔️  Attack for {} damage", damage);
    }
}

fn attack_hits(trigger: Trigger<Attack>, name: Query<&Name>) {
    if let Ok(name) = name.get(trigger.target()) {
        info!("Attack hit {}", name);
    }
}
