use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;
use rand::thread_rng;

use crate::game::component::Velocity;
use crate::game::drops::component::*;
use crate::game::enemy::SPAWN_DISTANCE;
use crate::game::enemy::component::*;
use crate::game::player::PLAYER_SPEED;
use crate::game::weapon::component::*;
use crate::my_assets::MyAssets;

pub fn spawn_red1(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    angle: f32,
    distance_offset: f32,
    mut bonus_drop: Option<Drop>,
) {
    let mut drops = vec![
        Drop::Experience(1),
    ];
    if let Some(drop) = bonus_drop.take() {
        drops.push(drop);
    }
    let hp = 4 + (20 + difficulty) * difficulty;
    let speed_modifier = 1. + difficulty as f32 / 3.;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                center.x + angle.cos() * (SPAWN_DISTANCE + distance_offset),
                center.y + angle.sin() * (SPAWN_DISTANCE + distance_offset),
                10.
            ).with_scale(Vec3::splat(1. + difficulty as f32 / 10.)),
            texture: assets.simple_enemy.clone(),
            ..Default::default()
        },
        Enemy,
        Hp(hp),
        Velocity {
            speed: Vec2::new(-angle.cos(), -angle.sin()) * PLAYER_SPEED * 0.3 * speed_modifier,
        },
        MovementPattern::StraightApproach(StraightApproach {
            turn_speed: PI / 2.,
            turnaround_distance: SPAWN_DISTANCE / 4.,
        }),
        MainCannon::new(1 + difficulty, 3.),
        Drops {
            score: 50,
            drops,
        },
    ));
}

pub fn spawn_red1_squad(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    wing_length: usize,
    bonus_drop: Option<Drop>,
) {
    let angle = thread_rng().gen_range(0.0..(2. * PI));
    spawn_red1(commands, center, assets, difficulty, angle, 0., bonus_drop);
    for i in 1..=wing_length {
        let i = i as f32;
        spawn_red1(commands, center, assets, difficulty, angle + 0.15 * i, 150. * i, None);
        spawn_red1(commands, center, assets, difficulty, angle - 0.15 * i, 150. * i, None);
    }
}

pub fn spawn_red4(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    angle: f32,
    distance_offset: f32,
    mut bonus_drop: Option<Drop>,
) {
    let mut drops = vec![
        Drop::Experience(1),
    ];
    if let Some(drop) = bonus_drop.take() {
        drops.push(drop);
    }
    // 4, 15, 28, 43, 60, 79
    let hp = 4 + (10 + difficulty) * difficulty;
    let speed_modifier = 1. + difficulty as f32 / 3.;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                center.x + angle.cos() * (SPAWN_DISTANCE + distance_offset),
                center.y + angle.sin() * (SPAWN_DISTANCE + distance_offset),
                10.
            ).with_scale(Vec3::splat(1. + difficulty as f32 / 10.)),
            texture: assets.simple_enemy2.clone(),
            ..Default::default()
        },
        Enemy,
        Hp(hp),
        Velocity {
            speed: Vec2::splat(0.),
        },
        MovementPattern::KeepDistance(KeepDistance {
            max_speed: PLAYER_SPEED * 0.5 * speed_modifier,
            target_distance: thread_rng().gen_range(300.0..400.0),
        }),
        StarCannon::new(2 + difficulty * 2, 1. + 3. / (1. + difficulty as f32 / 2.)),
        Drops {
            score: 75,
            drops,
        },
    ));
}

pub fn spawn_red4_squad(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    wing_length: usize,
    bonus_drop: Option<Drop>,
) {
    let angle = thread_rng().gen_range(0.0..(2. * PI));
    spawn_red4(commands, center, assets, difficulty, angle, 0., bonus_drop);
    for i in 1..=wing_length {
        let i = i as f32;
        spawn_red4(commands, center, assets, difficulty, angle + 0.15 * i, 150. * i, None);
        spawn_red4(commands, center, assets, difficulty, angle - 0.15 * i, 150. * i, None);
    }
}

pub fn spawn_red4_ring(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    difficulty: usize,
    count: usize,
    bonus_drop: Option<Drop>,
) {
    let angle = thread_rng().gen_range(0.0..(2. * PI));
    spawn_red4(commands, center, assets, difficulty, angle, 0., bonus_drop);
    for i in 1..count {
        let i = i as f32;
        spawn_red4(commands, center, assets, difficulty, angle + 2. * PI * i / count as f32, 0., None);
    }
}

