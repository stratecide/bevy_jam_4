use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{my_assets::MyAssets, game::weapon::component::MainCannon};

use super::{component::*, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: assets.player.clone(),
            ..Default::default()
        },
        Player {},
        PlayerFriend,
        MainCannon::new(4, 1.),
    ));
}

pub fn player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
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
        
        if let Some(cursor) = cursor_query.get_single().ok()
        .and_then(|w| w.cursor_position())
        .and_then(|c| camera_query.get_single().ok()
            .and_then(|(camera, camera_transform)| camera.viewport_to_world_2d(camera_transform, c))
        ) {
            let dir = cursor - transform.translation.xy();
            if dir.length() >= 1. {
                transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), (-dir.x).atan2(dir.y));
            }
        }
    }
}
