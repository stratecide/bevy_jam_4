use bevy::prelude::*;

use crate::game::component::*;
use crate::game::player::component::*;

use super::component::*;

pub fn despawn_dead(
    mut commands: Commands,
    entity_query: Query<(Entity, &Hp)>,
) {
    for (entity, hp) in entity_query.iter() {
        if hp.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_enemy_velocity(
    mut enemy_query: Query<(&mut Velocity, &mut Transform, &MovementPattern), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
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
                    transform.rotation = Quat::from_axis_angle(Vec3::Z, (-velocity.speed.x).atan2(velocity.speed.y));
                }
            }
        }
    }
}
