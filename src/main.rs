use bevy::prelude::*;
use systems::player;
use crate::systems::{camera, world};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::setup)
        .add_systems(Update, (player::move_player, camera::update_camera, world::world_limit))
        .run();
}
