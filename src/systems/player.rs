use bevy::prelude::*;
use crate::components::Player;

// S
pub fn move_player(mut query: Query<&mut Transform, With<Player>>, keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let speed = 200.0;
    for mut transform in &mut query {
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += speed * time.delta_secs();
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // gets the image and loads it, and scale the image down to 0.1
    commands.spawn((Sprite::from_image(asset_server.load("knight.png")), Player, Transform::from_scale(Vec3::splat(0.1))));
    commands.spawn((Sprite::from_image(asset_server.load("background.png")), Transform::from_xyz(0.0, 0.0, -1.0)));
}
