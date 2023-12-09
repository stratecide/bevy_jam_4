use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;
use rand::thread_rng;
use crate::game::component::FadeAway;
use crate::game::component::Velocity;
use crate::game::enemy::component::*;
use crate::game::enemy::resource::EnemyUpgrades;
use crate::game::player::component::*;
use crate::game::player::resource::Upgrades;
use crate::my_assets::MyAssets;

use super::Weapon;
use super::component::*;

pub fn despawn_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<Bullet>>,
    time: Res<Time>,
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
    let dt = time.delta_seconds();
    for (entity, mut bullet, bullet_transform) in bullet_query.iter_mut() {
        let pos = bullet_transform.translation.xy();
        if (pos.x - camera_translation.x).abs() > camera_size.x / 2. + 10.
        || (pos.y - camera_translation.y).abs() > camera_size.y / 2. + 10. {
            bullet.despawn_timer += dt;
            if bullet.despawn_timer >= 10. {
                commands.entity(entity).despawn();
            }
        } else {
            bullet.despawn_timer = 0.;
        }
    }
}

pub fn tick_weapons<W: Weapon>(
    mut commands: Commands,
    mut weapon_query: Query<(&mut WeaponCooldown<W>, &W, &Transform, Option<&PlayerFriend>)>,
    assets: Res<MyAssets>,
    player_upgrades: Res<Upgrades>,
    enemy_upgrades: Res<EnemyUpgrades>,
    time: Res<Time>,
) {
    for (mut cooldown, weapon, transform, friendly) in weapon_query.iter_mut() {
        cooldown.cooldown -= time.delta_seconds();
        if cooldown.cooldown <= 0. {
            let upgrades = if friendly.is_some() {
                &player_upgrades.0
            } else {
                &enemy_upgrades.0
            };
            cooldown.cooldown = 0.0_f32.max(cooldown.cooldown + weapon.max_cooldown(upgrades));
            weapon.fire(&mut commands, transform, upgrades, friendly.is_some(), &assets);
        }
    }
}

pub fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in bullet_query.iter_mut() {
        transform.translation.x += velocity.speed.x * time.delta_seconds();
        transform.translation.y += velocity.speed.y * time.delta_seconds();
        transform.rotation = Quat::from_axis_angle(Vec3::Z, (-velocity.speed.x).atan2(velocity.speed.y));
    }
}

fn collision_point(pos1: Vec2, pos2: Vec2, other:Vec2, distance: f32) -> Option<f32> {
    if other.distance(pos2) < distance {
        let mut low = 0.;
        let mut high = 1.;
        let mut between = 0.5;
        for _ in 0..8 {
            between = (low + high) / 2.;
            if other.distance(pos1 * between + pos2 * (1. - between)) < distance {
                high = between;
            } else {
                low = between;
            }
        }
        Some(between)
    } else {
        None
    }
}

pub fn enemy_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Velocity), (With<Bullet>, With<PlayerFriend>)>,
    mut enemy_query: Query<(&mut Hp, &Transform, &Handle<Image>), (With<Enemy>, Without<PlayerFriend>)>,
    images: Res<Assets<Image>>,
    assets: Res<MyAssets>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();
    let dt = time.delta_seconds();
    for (bullet_entity, bullet_transform, velocity) in bullet_query.iter() {
        let mut offset = None;
        for (mut hp, enemy_transform, img) in enemy_query.iter_mut() {
            if let Some(o) = collision_point(bullet_transform.translation.xy() - velocity.speed * dt, bullet_transform.translation.xy(), enemy_transform.translation.xy(), 6. * bullet_transform.scale.x + images.get(img).unwrap().height() as f32 * 0.4 * enemy_transform.scale.x) {
                offset = Some(o.min(offset.unwrap_or(0.)));
                hp.0 = 1.max(hp.0) - 1;
            }
        }
        if let Some(offset) = offset {
            commands.entity(bullet_entity).despawn();
            let pos = bullet_transform.translation.xy() - (1. - offset) * velocity.speed * dt;
            let mut transform = Transform::from_xyz(pos.x, pos.y, 2.);
            transform.rotate_z(rng.gen_range(0.0..PI));
            transform.scale *= rng.gen_range(0.8..1.);
            commands.spawn((
                SpriteBundle {
                    transform,
                    texture: assets.bullet_explosion.clone(),
                    ..Default::default()
                },
                Velocity {
                    speed: velocity.speed * 0.2,
                },
                FadeAway::new(rng.gen_range(0.1..0.2)),
            ));
        }
    }
}

pub fn player_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Velocity), (With<Bullet>, Without<PlayerFriend>)>,
    mut player_query: Query<(Entity, &mut Vulnerability, &Transform), With<PlayerFriend>>,
    mut upgrades: ResMut<Upgrades>,
    assets: Res<MyAssets>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();
    let dt = time.delta_seconds();
    for (bullet_entity, bullet_transform, velocity) in bullet_query.iter() {
        let mut offset = None;
        for (player, mut vulnerability, player_transform) in player_query.iter_mut() {
            if !vulnerability.vulnerable() {
                continue;
            }
            if let Some(o) = collision_point(bullet_transform.translation.xy() - velocity.speed * dt, bullet_transform.translation.xy(), player_transform.translation.xy(), 6. * bullet_transform.scale.x + 4. * player_transform.scale.x) {
                offset = Some(o.min(offset.unwrap_or(0.)));
                let lives = upgrades.get(Upgrade::ExtraLife);
                vulnerability.reset();
                if lives > 0 {
                    upgrades.0.insert(Upgrade::ExtraLife, lives - 1);
                } else {
                    commands.entity(player).despawn();
                }
            }
        }
        if let Some(offset) = offset {
            commands.entity(bullet_entity).despawn();
            let pos = bullet_transform.translation.xy() - (1. - offset) * velocity.speed * dt;
            let mut transform = Transform::from_xyz(pos.x, pos.y, 2.);
            transform.rotate_z(rng.gen_range(0.0..PI));
            transform.scale *= rng.gen_range(0.8..1.);
            commands.spawn((
                SpriteBundle {
                    transform,
                    texture: assets.bullet_explosion.clone(),
                    ..Default::default()
                },
                Velocity {
                    speed: velocity.speed * 0.2,
                },
                FadeAway::new(rng.gen_range(0.1..0.2)),
            ));
        }
    }
}
