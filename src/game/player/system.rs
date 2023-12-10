use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{my_assets::MyAssets, game::{weapon::component::*, ZOOM}};

use super::{component::*, PLAYER_SPEED, resource::Upgrades};

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
        PlayerMovement,
        MainCannon::new(0, 0.3),
        StarCannon::new(0, 1.),
        Vulnerability::new(),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(250., 0., 20.).with_scale(Vec3::splat(0.5)),
            texture: assets.ufo.clone(),
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        PlayerFriend,
        PlayerMovement,
        PlayerSatellite(-0.5),
        SpiralCannon::new(0, 1., 0.3, 0, vec![(Vec2::splat(0.), false)]),
    ));
}

pub fn player_wasd(
    mut player_query: Query<&mut Transform, With<PlayerMovement>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    upgrades: Res<Upgrades>,
) {
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
        for mut transform in player_query.iter_mut() {
            transform.translation += dir.normalize() * PLAYER_SPEED * time.delta_seconds() * (1. + upgrades.get(Upgrade::MovementSpeed) as f32 * MOVEMENT_SPEED_BONUS);
        }
    }
}

pub fn player_cursor(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<Player>>,
    cursor_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
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

pub fn player_satellite(
    player_query: Query<&Transform, (With<Camera>, Without<PlayerSatellite>)>,
    mut satellite_query: Query<(&mut Transform, &PlayerSatellite)>,
    time: Res<Time>,
) {
    if let Ok(center) = player_query.get_single() {
        for (mut transform, sat) in satellite_query.iter_mut() {
            let distance = center.translation.xy() - transform.translation.xy();
            let angle = distance.y.atan2(distance.x);
            let destination = center.translation.xy() - Vec2::from_angle(angle + sat.0 * time.delta_seconds()) * distance.length();
            transform.translation.x = destination.x;
            transform.translation.y = destination.y;
        }
    }
}

pub fn make_player_satellite_visible(
    mut satellite_query: Query<&mut Sprite, (With<PlayerSatellite>, With<PlayerFriend>)>,
    time: Res<Time>,
    upgrades: Res<Upgrades>,
) {
    if upgrades.get(Upgrade::SpiralBulletCount) > 0 {
        for mut sprite in satellite_query.iter_mut() {
            let alpha = sprite.color.a() + time.delta_seconds();
            sprite.color.set_a(alpha.min(1.));
        }
    }
}

pub fn tick_vulnerability(
    mut player_query: Query<(&mut Vulnerability, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    for (mut vulnerability, mut sprite) in player_query.iter_mut() {
        vulnerability.tick(time.delta_seconds());
        if vulnerability.vulnerable() {
            sprite.color = Color::WHITE;
        } else {
            sprite.color.set_a((vulnerability.remaining_seconds() * 20.).sin() * 0.5 + 0.5);
        }
    }
}
