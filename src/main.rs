use bevy::prelude::*;

mod components;
mod systems;
use systems::player;

use crate::systems::camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::setup)
        .add_systems(Update, (player::move_player, camera::update_camera))
        .run();
}
