use bevy::prelude::*;

// A basic system with no dependencies
fn hello_world() {
    // Print out hello world lol
    println!("Hello world!");
}

// To create a component we just have to derive the component trait
#[derive(Component)]
struct Person;

// Components can have many different types of values inside
#[derive(Component)]
struct Name(String);

// System to spawn our entities
//
// Systems can request mostly anything into their function arguments. Bevy will
// try to magically pull these into the function when it executes
//
// In this one, the "Commands" resource is pulled. This is a special resource
// that lets you spawn entities, add components, remove components, create
// or replace resources and run systems
fn add_people(mut commands: Commands) {
    commands
        // Creates an entity with the Person component
        .spawn(Person)
        // Adds or replaces the Name component
        .insert(Name("Pedro".to_string()));
    // This one will create the entity with only one function call
    commands.spawn((Person, Name("Sergio".to_string())));
    // You can also do some more advanced stuff
    let mut entity = commands.spawn(Person);
    entity.insert(Name("Karl".to_string()));
}

// System to list out our entites
//
// Again we can pull any resource into our system. In this one, we use a
// special type to pull any entity that matches our Query. This has some
// restrictions that you might find in the future.
//
// The basic is that you can ask for something like this: `Query<&Position>`
// for just any entity with a position component but this won't let you modify
// this component, you'd have to ask for `Query<&mut Position>` to be able to
// modify the component.
//
// Since we want to greet every Person by their name, we ask for
// `Query<&Name, With<Person>>`. Since we don't need the "Person" component's
// data we can use the "With<T>" type to filter out entities that have "Name"
// and "Person".
fn greet_people(query: Query<&Name, With<Person>>) {
    // It's really simple to run through our entities, just a simple for...in
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

// System to fix Sérgio's name
//
// Clearly, Sérgio's name is misspelled in our spawn system, so we have to
// fix that.
fn fix_sergio_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Sergio" {
            name.0 = "Sérgio".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
        // adding the hello_world system into the Update schedule will run this
        // system on every tick
        .add_systems(Update, hello_world)
        // adding to the Startup schedule will run only at the start of the
        // program. By using "chain", it makes sure that these will run
        // sequentially!
        .add_systems(Startup, (add_people, greet_people).chain())
        // adding fix_sergio_name into the Startup schedule might make it run
        // before or after add_people, so sometimes it might not fix the name.
        // Calling "after" will have a similar effect to "chain" where the
        // system will run after the other has completed
        .add_systems(Startup, fix_sergio_name.after(add_people))
        .run();
}
