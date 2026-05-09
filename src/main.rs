use bevy::prelude::*;

mod components;
mod systems;
use systems::player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::setup)
        .add_systems(Update, player::move_player)
        .run();
}
