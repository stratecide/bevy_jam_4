use bevy::prelude::*;
use crate::game::player::component::PlayerFriend;
use crate::my_assets::MyAssets;

use super::Weapon;
use super::component::*;

pub fn despawn_offscreen(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(c) => c,
        _ => return,
    };
    let camera_size = match camera.logical_viewport_size() {
        Some(s) => s,
        _ => return,
    };
    let camera_translation = camera_transform.translation().xy();
    for (bullet, bullet_transform) in bullet_query.iter() {
        let pos = bullet_transform.translation.xy();
        if (pos.x - camera_translation.x).abs() > camera_size.x / 2. + 10.
        || (pos.y - camera_translation.y).abs() > camera_size.y / 2. + 10. {
            commands.entity(bullet).despawn();
        }
    }
}

pub fn tick_weapons<W: Weapon>(
    mut commands: Commands,
    mut player_query: Query<(&mut WeaponCooldown<W>, &W, &Transform, Option<&PlayerFriend>)>,
    assets: Res<MyAssets>,
    time: Res<Time>,
) {
    for (mut cooldown, weapon, transform, friendly) in player_query.iter_mut() {
        cooldown.cooldown -= time.delta_seconds();
        if cooldown.cooldown <= 0. {
            cooldown.cooldown = weapon.max_cooldown();
            weapon.fire(&mut commands, transform, friendly.is_some(), &assets);
        }
    }
}

pub fn tick_bullets(
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
    time: Res<Time>,
) {
    for mut transform in bullet_query.iter_mut() {
        let direction = transform.local_y();
        transform.translation += direction * MAIN_BULLET_SPEED * time.delta_seconds();
    }
}
