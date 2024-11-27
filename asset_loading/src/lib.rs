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

// System to close on escape
fn exit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

pub struct SimpleGamePlugin;

impl Plugin for SimpleGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_people, fix_sergio_name, greet_people).chain())
            .add_systems(Update, exit_on_escape);
    }
}
