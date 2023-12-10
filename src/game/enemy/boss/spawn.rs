use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;
use rand::thread_rng;

use crate::game::component::Velocity;
use crate::game::drops::component::*;
use crate::game::enemy::SPAWN_DISTANCE;
use crate::game::enemy::component::*;
use crate::game::player::PLAYER_SPEED;
use crate::game::player::component::PlayerMovement;
use crate::game::weapon::component::*;
use crate::my_assets::MyAssets;

pub fn spawn_boss(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    angle: f32,
    distance_offset: f32,
    mut bonus_drop: Option<Drop>,
) {
    let mut drops = vec![
        Drop::Experience(10),
    ];
    if let Some(drop) = bonus_drop.take() {
        drops.push(drop);
    }
    let hp = 200 + (50 + difficulty) * difficulty * 20;
    let speed_modifier = 1. + difficulty as f32 / 3.;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                center.x + angle.cos() * (SPAWN_DISTANCE + distance_offset),
                center.y + angle.sin() * (SPAWN_DISTANCE + distance_offset),
                10.
            ).with_scale(Vec3::splat(1. + difficulty as f32 / 10.)),
            texture: assets.boss.clone(),
            ..Default::default()
        },
        Enemy,
        PlayerMovement,
        Hp(hp),
        Velocity {
            speed: Vec2::new(-angle.cos(), -angle.sin()) * PLAYER_SPEED * 0.3 * speed_modifier,
        },
        MovementPattern::Hover(Hover {
            angular_speed: thread_rng().gen_range(-(PI / 10.)..=(PI / 10.)),
            target_distance: SPAWN_DISTANCE / 4.,
        }),
        MainCannon::new(3, 8. / (4. + difficulty as f32)),
        SpiralCannon::new(3 + difficulty * 2, 1. / 4., 2., 4 + difficulty * 2, vec![
            (Vec2::new(75., 30.), true),
            (Vec2::new(-75., 30.), false),
        ]),
        Drops {
            score: 5000,
            drops,
        },
    ));
}

pub fn spawn_boss_simple(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
) {
    spawn_boss(commands, center, assets, difficulty, thread_rng().gen_range(0.0..(2. * PI)), 0., None)
}
