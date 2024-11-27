use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// System to spawn our entities
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Pedro".to_string())));
    commands.spawn((Person, Name("Sergio".to_string())));
    commands.spawn((Person, Name("Karl".to_string())));
}

// System to list out our entites
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

// System to fix Sérgio's name
fn fix_sergio_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Sergio" {
            name.0 = "Sérgio".to_string();
            break;
        }
    }
}

// Since we have the window plugin now, let's make a quick system to exit
// the game after pressing the "Escape" button
//
// This introduces 2 new parameter types, the Res (Resource) and EventWriter
// (and EventReader, not seen in this function). The Res will request a
// structure that is not part of a component and lives globally in the world.
// The EventWriter is for sending events to EventReaders, which can also be
// any enum in our program.
//
// You can read about these here https://bevy-cheatbook.github.io/programming/res.html
// and here https://bevy-cheatbook.github.io/programming/events.html
fn exit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

// This is what a basic plugin looks like, it's simply a type for the name of
// the plugin and a implementation of the "build" function to add all systems
// and resources
pub struct SimpleGamePlugin;

impl Plugin for SimpleGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_people, fix_sergio_name, greet_people).chain())
            .add_systems(Update, exit_on_escape);
    }
}
