use std::f32::consts::PI;

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

use crate::game::enemy::SPAWN_DISTANCE;
use crate::game::enemy::component::*;
use crate::game::player::component::Player;
use crate::my_assets::MyAssets;

use super::component::*;

pub fn spawn_asteroids(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, With<Player>>,
    assets: Res<MyAssets>,
) {
    let player_transform = match player_query.get_single() {
        Ok(c) => c,
        _ => return,
    };
    let player_translation = player_transform.translation().xy();
    let mut rng = thread_rng();
    if rng.gen_bool(0.01) {
        let angle = rng.gen_range(0.0..(2. * PI));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    player_translation.x + angle.cos() * SPAWN_DISTANCE,
                    player_translation.y + angle.sin() * SPAWN_DISTANCE,
                    8.
                ),
                texture: assets.asteroids.choose(&mut rng).cloned().unwrap(),
                ..Default::default()
            },
            Enemy,
            Hp(rng.gen_range(1..=5)),
            Tumble::new(&mut rng),
        ));
    }
}

pub fn move_asteroids(
    mut asteroid_query: Query<(&mut Transform, &Tumble), Without<Player>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, tumble) in asteroid_query.iter_mut() {
        transform.rotate_axis(Vec3::Z, tumble.angular * time.delta_seconds());
        transform.translation.x += tumble.speed.x * time.delta_seconds();
        transform.translation.y += tumble.speed.y * time.delta_seconds();
        if let Ok(player) = player_query.get_single() {
            let player_translation = player.translation().xy();
            let dir = player_translation - transform.translation.xy();
            if dir.length() > 2. * SPAWN_DISTANCE {
                let dir = dir.normalize();
                transform.translation.x += dir.x * 3. * SPAWN_DISTANCE;
                transform.translation.y += dir.y * 3. * SPAWN_DISTANCE;
            }
        }
    }
}
