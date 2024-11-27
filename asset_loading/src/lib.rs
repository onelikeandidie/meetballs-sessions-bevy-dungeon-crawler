use bevy::prelude::*;

// SystemSets are a way to make it so that game systems only run when a
// condition is met. We use these to separate systems that run in the
// in-game, loading and paused states. Oh btw there's states.
//
// Here we define our possible game states. You're not limited to 1 game state,
// you can have different game states at the same time, like a Multiplayer
// state or a Music state. Whatever you really want.
//
// States have to derive all these traits to work correctly, dw the rust error
// messages will tell you every time you forget! Every... single... time...
#[derive(States, PartialEq, Eq, Hash, Clone, Debug)]
enum MyGameState {
    Loading,
    Running,
    Paused,
}

// These are our system sets, they also have to derive all these traits
#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct AssetLoadingSet;

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct RunningSet;

#[derive(SystemSet, PartialEq, Eq, Hash, Clone, Debug)]
struct PausedSet;

// This resource is important to keep track of loaded assets as well as define
// their types
//
// To know more about what types Bevy can load with it's default plugins, you
// can have a look here https://bevy-cheatbook.github.io/builtins.html?highlight=asset#assets
#[derive(Resource)]
struct GameAssets {
    rustacean: Handle<Image>,
    font: Handle<Font>,
}

// This system will load our game assets and change to the running state when
// done.
fn load_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    // When we use ResMut, we request a mutable version of that resource
    mut game_state: ResMut<NextState<MyGameState>>,
) {
    // Here we load the image of the crab mascot of rust. We don't need any
    // type anotations because we already declare our types in our GameAssets
    // resource
    let rustacean = asset_server.load("sprites/rustacean-flat-noshadow.png");
    // Load font too for UI later
    let font = asset_server.load("fonts/InterDisplay-Medium.ttf");
    commands.insert_resource(GameAssets { rustacean, font });
    game_state.set(MyGameState::Running);
}

// This system will spawn our mascot and setup our camera
fn setup_world(mut commands: Commands, assets: Res<GameAssets>) {
    // Bundles are presets of multiple components together, this will spawn
    // an entity with multiple components
    commands
        .spawn(Camera2dBundle::default())
        // Set this camera to be the default UI render camera
        .insert(IsDefaultUiCamera);
    commands.spawn(SpriteBundle {
        texture: assets.rustacean.clone(),
        transform: Transform::from_translation(Vec3::new(-128.0, 0.0, 0.0)),
        sprite: Sprite {
            // We can adjust the colour channels if we want
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            // Render a specific size
            custom_size: Some(Vec2::new(100.0, 100.0)),
            // Render a specific section
            rect: Some(Rect::new(48.0, 48.0, 200.0, 200.0)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: assets.rustacean.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::linear_rgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: assets.rustacean.clone(),
        transform: Transform::from_translation(Vec3::new(128.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::linear_rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    // We will spin this entity around to show pause and unpause.
    // So we want to mark this with a component so we can filter them in
    // a system
    commands
        .spawn(AliveFerris { speed: 90.0 })
        .insert(SpriteBundle {
            texture: assets.rustacean.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 128.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            ..Default::default()
        });
}

// System to move the camera with the arrow keys
fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    // The time resource will help with keeping speed consistent
    time: Res<Time>,
) {
    // This way, you get 1 from the query. If there is more than one, the game
    // will crash, so make sure you use this only when you know there's only
    // going to be one. There is a safer version of this "get_single_mut" but
    // we know in this instance it won't crash
    let mut camera_transform = query.single_mut();
    let delta_time = time.delta_seconds();
    // The reason why we put the direction in a separate variable is to make
    // sure we normalize the vector to ensure we have the same movement speed
    // in diagonals as as in vertical and horizontal directions
    let mut movement = Vec3::ZERO;
    if keys.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    if movement.length() == 0.0 {
        return;
    }
    let movement = movement.normalize();
    let translation = camera_transform.translation + movement * 128.0 * delta_time;
    camera_transform.translation = translation;
}

// System to handle pause and unpause

#[derive(Component)]
struct AliveFerris {
    speed: f32,
}

// System to spin AliveFerris
//
// In this system, we need to modify the rotation and need to know what the
// rotation speed is for this ferris. We can actually request a combination
// of mutable and unmutable components on an entity like below.
fn spin_alive_ferris(mut query: Query<(&mut Transform, &AliveFerris)>, time: Res<Time>) {
    for (mut transform, alive_ferris) in &mut query {
        transform.rotate_z((alive_ferris.speed * time.delta_seconds()).to_radians());
    }
}

// System to pause and unpause the game
fn pause_unpause(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<MyGameState>>,
    mut next_state: ResMut<NextState<MyGameState>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    if *current_state.get() == MyGameState::Running {
        next_state.set(MyGameState::Paused);
    } else {
        next_state.set(MyGameState::Running);
    }
}

#[derive(Component)]
struct PauseMenu;

// System to create the pause menu
//
// The UI Framework included with bevy works similar to html and css, this is
// because Bevy can compile to WASM and run on WebGL. To look at more examples
// you can follow the bevy examples on their site
// https://bevyengine.org/examples/#ui-user-interface
fn open_pause_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(PauseMenu)
        .insert(NodeBundle {
            // This is equivalent to css'
            //
            // element {
            //     width: 100%;
            //     height: 100%;
            //     display: flex;
            //     align-items: center;
            //     justify-content: center;
            // }
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Node with just text with our font
            parent.spawn((
                TextBundle::from_section(
                    "paused",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                Label,
            ));
        });
}

// System to destroy the pause menu
fn close_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    let menu_entity = query.single();
    // This is how you despawn entities
    // You can despawn just the parent entity with "despawn()"
    commands.entity(menu_entity).despawn_recursive();
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
        app.insert_state(MyGameState::Loading)
            // Here we configure what sets can run at what time for Startup
            // and Update schedules
            .configure_sets(
                Startup,
                (
                    AssetLoadingSet.run_if(in_state(MyGameState::Loading)),
                    RunningSet.run_if(in_state(MyGameState::Running)),
                    PausedSet.run_if(in_state(MyGameState::Paused)),
                ),
            )
            .configure_sets(
                Update,
                (
                    AssetLoadingSet.run_if(in_state(MyGameState::Loading)),
                    RunningSet.run_if(in_state(MyGameState::Running)),
                    PausedSet.run_if(in_state(MyGameState::Paused)),
                ),
            )
            .add_systems(
                // OnEnter and OnExit will make a system run when that state is triggered
                OnEnter(MyGameState::Loading),
                load_assets.in_set(AssetLoadingSet),
            )
            .add_systems(
                // Here we use OnExit because if the player pauses the game
                // the OnEnter would trigger the world setup again and again
                OnExit(MyGameState::Loading),
                setup_world.in_set(RunningSet),
            )
            // The pause system needs to be able to run in both the paused and running states
            .add_systems(Update, pause_unpause.in_set(RunningSet))
            .add_systems(Update, pause_unpause.in_set(PausedSet))
            // Open and close systems
            .add_systems(OnEnter(MyGameState::Paused), open_pause_menu)
            .add_systems(OnExit(MyGameState::Paused), close_pause_menu)
            // These systems only run in the Running state
            .add_systems(Update, move_camera.in_set(RunningSet))
            .add_systems(Update, spin_alive_ferris.in_set(RunningSet))
            // This system can run at any time, independent of set
            .add_systems(Update, exit_on_escape);
    }
}
