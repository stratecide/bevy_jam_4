use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{my_assets::MyAssets, game::{weapon::component::MainCannon, ZOOM}};

use super::{component::*, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 20.),
            texture: assets.player.clone(),
            ..Default::default()
        },
        Player {},
        PlayerFriend,
        MainCannon::new(1, 0.3),
    ));
}

pub fn player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<Player>>,
    cursor_query: Query<&Window, With<PrimaryWindow>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut dir = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            dir.y += 1.;
        }
        if input.pressed(KeyCode::A) {
            dir.x -= 1.;
        }
        if input.pressed(KeyCode::S) {
            dir.y -= 1.;
        }
        if input.pressed(KeyCode::D) {
            dir.x += 1.;
        }

        if dir != Vec3::ZERO {
            transform.translation += dir.normalize() * PLAYER_SPEED * time.delta_seconds();
        }
        
        if let Ok((camera, camera_global_transform)) = camera_query.get_single() {
            if let Some(cursor) = cursor_query.get_single().ok()
            .and_then(|w| w.cursor_position())
            .and_then(|c| camera.viewport_to_world_2d(camera_global_transform, c)) {
                let dir = cursor - transform.translation.xy();
                if dir.length() >= 1. {
                    transform.rotation = Quat::from_axis_angle(Vec3::Z, (-dir.x).atan2(dir.y));
                }
            }
        }
    }
}

pub fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    window_query: Query<&Window, (With<PrimaryWindow>, Without<Camera>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player) = player_query.get_single() {
            camera_transform.translation.x = player.translation.x;
            camera_transform.translation.y = player.translation.y;
        }
        if let Ok(window) = window_query.get_single() {
            let scale = ZOOM / (window.width() * window.height() / 1_000_000.).sqrt();
            camera_transform.scale.x = scale;
            camera_transform.scale.y = camera_transform.scale.x;
        }
    }
}
