use bevy::prelude::*;

use asset_loading::SimpleGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleGamePlugin)
        .run();
}
