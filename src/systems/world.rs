use bevy::prelude::*;
use crate::components::Player;


const WORLD_HEIGHT: f32 = 540.;
const WORLD_WIDTH: f32 = 960.;

pub fn world_limit(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = match player_query.single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    player_transform.translation.x = player_transform.translation.x.clamp(-WORLD_WIDTH, WORLD_WIDTH);
    player_transform.translation.y = player_transform.translation.y.clamp(-WORLD_HEIGHT, WORLD_HEIGHT);
} 

