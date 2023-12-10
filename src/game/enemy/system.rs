use bevy::prelude::*;

use crate::game::{component::*, increase_score};
use crate::game::drops::component::*;
use crate::game::player::component::*;
use crate::game::player::resource::Upgrades;
use crate::game::resource::{Score, WaveTimer};
use crate::my_assets::MyAssets;

use super::component::*;

pub fn despawn_dead(
    mut commands: Commands,
    entity_query: Query<(Entity, &Hp, &Transform, Option<&Drops>)>,
    assets: Res<MyAssets>,
    mut score: ResMut<Score>,
) {
    for (entity, hp, transform, drops) in entity_query.iter() {
        if hp.0 == 0 {
            commands.entity(entity).despawn();
            if let Some(drops) = drops {
                increase_score(&mut commands, drops.score, transform.translation.xy(), &mut score, &assets);
                for drop in &drops.drops {
                    commands.spawn((
                        drop.sprite(transform.translation.xy(), &assets),
                        *drop,
                        /*SpriteBundle {
                            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.).with_scale(Vec3::splat((drops.experience as f32 + 1.).ln())),
                            texture: assets.exp.clone(),
                            ..Default::default()
                        },*/
                    ));
                }
            }
        }
    }
}

pub fn update_enemy_velocity(
    mut enemy_query: Query<(&mut Velocity, &mut Transform, &MovementPattern)>,
    player_query: Query<&Transform, (With<Player>, Without<MovementPattern>)>,
    time: Res<Time>,
    wave_timer: Res<WaveTimer>,
) {
    let player = match player_query.get_single() {
        Ok(p) => p,
        _ => return,
    };
    for (mut velocity, mut transform, pattern) in enemy_query.iter_mut() {
        match pattern {
            MovementPattern::StraightApproach(data) => {
                let distance = player.translation.xy() - transform.translation.xy();
                if distance.length() > velocity.speed.length() && distance.dot(velocity.speed) > 0. || distance.length() >= data.turnaround_distance {
                    let mut angle_diff = velocity.speed.angle_between(distance);
                    if angle_diff.abs() > data.turn_speed * time.delta_seconds() {
                        angle_diff *= data.turn_speed * time.delta_seconds() / angle_diff.abs();
                    }
                    velocity.speed = velocity.speed.rotate(Vec2::from_angle(angle_diff));
                }
                transform.rotation = Quat::from_axis_angle(Vec3::Z, (-velocity.speed.x).atan2(velocity.speed.y));
            }
            MovementPattern::KeepDistance(data) => {
                let distance = player.translation.xy() - transform.translation.xy();
                let dir = if distance.length() < data.target_distance {
                    -distance.normalize()
                } else {
                    distance.normalize()
                };
                velocity.speed = dir * data.max_speed;
                let factor = (distance.length() - data.target_distance).abs() / data.target_distance * 3.;
                if factor < 1. {
                    velocity.speed *= factor;
                    let max_sideways = data.max_speed * (1. - factor * factor).sqrt();
                    let wave = dir.x.atan2(-dir.y) + wave_timer.0 / 10.;
                    velocity.speed += max_sideways * wave.sin() * Vec2::new(dir.y, -dir.x);
                }
                transform.rotation = Quat::from_axis_angle(Vec3::Z, (-velocity.speed.x).atan2(velocity.speed.y));
            }
            MovementPattern::Hover(data) => {
                let distance = player.translation.xy() - transform.translation.xy();
                transform.rotation = Quat::from_axis_angle(Vec3::Z, (-distance.x).atan2(distance.y));
                velocity.speed = velocity.speed.length() * distance.normalize();
                if distance.length() <= data.target_distance {
                    velocity.speed *= 0.;
                } else if (distance.length() - data.target_distance) < velocity.speed.length() * time.delta_seconds() {
                    let speed = velocity.speed.length() * time.delta_seconds();
                    velocity.speed *= (distance.length() - data.target_distance) / speed;
                };
            }
        }
    }
}

pub fn enemy_collisions(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Vulnerability, &Transform), With<PlayerFriend>>,
    mut enemy_query: Query<(&mut Hp, &Transform, &Handle<Image>), (With<Enemy>, Without<PlayerFriend>)>,
    images: Res<Assets<Image>>,
    mut upgrades: ResMut<Upgrades>,
) {
    for (player, mut vulnerability, player_transform) in player_query.iter_mut() {
        if !vulnerability.vulnerable() {
            continue;
        }
        for (mut hp, enemy_transform, img) in enemy_query.iter_mut() {
            if player_transform.translation.xy().distance(enemy_transform.translation.xy()) < 4. * player_transform.scale.x + images.get(img).unwrap().height() as f32 * 0.4 * enemy_transform.scale.x {
                // enemy damage
                hp.0 = 1.max(hp.0) - 1;
                // player damage
                let lives = upgrades.get(Upgrade::ExtraLife);
                vulnerability.reset();
                if lives > 0 {
                    upgrades.0.insert(Upgrade::ExtraLife, lives - 1);
                } else {
                    commands.entity(player).despawn();
                }
            }
        }
    }
}
