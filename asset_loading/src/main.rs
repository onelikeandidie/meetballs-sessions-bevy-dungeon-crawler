use bevy::prelude::*;

use refactored_project::SimpleGamePlugin;

fn main() {
    App::new()
        // This will actually open a window, it's the render, scheduler, asset
        // loader and all the other basic features Bevy has
        .add_plugins(DefaultPlugins)
        // Our systems got refactored into a Plugin in lib.rs
        .add_plugins(SimpleGamePlugin)
        .run();
}
