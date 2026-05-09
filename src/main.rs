use bevy::prelude::*;

mod components;
mod events;
mod states;
mod systems;

use states::GameState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, (
            camera::setup_camera,
            player::setup_player,
            world::setup_world,
            ui::setup_ui,
        ))
        .add_systems(Update, (
            player::handle_player_input,
            player::update_player,
            world::update_world,
            camera::update_camera,
            ui::update_ui,
        ))
        .run();
}
