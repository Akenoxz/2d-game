use bevy::prelude::*;
use crate::components::Player;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Component)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[allow(dead_code)]
#[derive(Component)]
enum AnimationState {
    Idle,
    Walking
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}


// S
pub fn move_player(mut query: Query<&mut Transform, With<Player>>, keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let speed = 200.0;
    for mut transform in &mut query {
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * time.delta_secs();
        } else if keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * time.delta_secs();
        }
    }
}

// Borrowed function
// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
                config.frame_timer= AnimationConfig::timer_from_fps(config.fps);
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>) {

    let texture = asset_server.load("sprites/character/IDLE/idle_down.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(96,80), 8, 1, None, None);
    let layout_handle = texture_atlas_layout.add(layout);
    commands.spawn(Camera2d);
    // gets the image and loads it, and scale the image down to 0.1
    commands.spawn((
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas { layout: layout_handle, index: 0 }), 
        Player,
        Direction::Down,
        AnimationState::Idle,
        AnimationConfig::new(0, 7, 10),
        Transform::from_scale(Vec3::splat(2.))));
    commands.spawn((Sprite::from_image(asset_server.load("background.png")), Transform::from_xyz(0.0, 0.0, -1.0)));
}
