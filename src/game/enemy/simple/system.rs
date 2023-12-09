use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::game::component::Velocity;
use crate::game::drops::component::*;
use crate::game::enemy::SPAWN_DISTANCE;
use crate::game::enemy::component::*;
use crate::game::player::PLAYER_SPEED;
use crate::game::player::component::Player;
use crate::game::weapon::component::*;
use crate::my_assets::MyAssets;

pub fn spawn_small_enemy(
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
                    10.
                ),
                texture: assets.simple_enemy.clone(),
                ..Default::default()
            },
            Enemy,
            Hp(4),
            Velocity {
                speed: Vec2::new(-angle.cos(), -angle.sin()) * PLAYER_SPEED * 0.3,
            },
            MovementPattern::StraightApproach(StraightApproach {
                turn_speed: PI / 2.,
                turnaround_distance: SPAWN_DISTANCE / 4.,
            }),
            MainCannon::new(1, 3.),
            Drops {
                experience: 1,
                score: 50,
            },
        ));
    }
}
