use bevy::prelude::*;
use crate::components::Player;

const CAMERA_DECAY_RATE: f32 = 1.;

pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    // destructuring, could be done like that 
    // let x = player.translation.x;
    // let y = player.translation.y;
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    //applies a smooth effect through smooth_nudge which delays by the value
    //of camera_decay_rate 
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}