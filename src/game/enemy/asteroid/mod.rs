pub mod component;
mod system;
use std::f32::consts::PI;

use component::*;
use system::*;

use bevy::prelude::*;
use rand::Rng;
use rand::{thread_rng, seq::SliceRandom};

use crate::game::GameSystems;
use crate::game::player::PLAYER_SPEED;
use crate::game::drops::component::*;
use crate::my_assets::MyAssets;

use super::SPAWN_DISTANCE;
use super::component::*;

pub const MAX_ASTEROID_SPEED: f32 = PLAYER_SPEED * 0.5;

#[derive(Debug)]
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_systems(FixedUpdate, spawn_asteroids
        //    .in_set(GameSystems::SpawnEnemy))
        .add_systems(FixedUpdate, move_asteroids
            .in_set(GameSystems::ShipMovement))
        ;
    }
}

pub fn spawn_debris(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    special_drop: Drop,
    extra_asteroids: usize,
) {
    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0..(2. * PI));
    spawn_asteroid(commands, center, assets, angle, special_drop);
    for _ in 0..extra_asteroids {
        spawn_asteroid(commands, center, assets, angle, Drop::Coin);
    }
}

pub fn spawn_asteroid(
    commands: &mut Commands,
    center: Vec2,
    assets: &MyAssets,
    angle: f32,
    drop: Drop,
) {
    let mut rng = thread_rng();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                center.x + angle.cos() * SPAWN_DISTANCE,
                center.y + angle.sin() * SPAWN_DISTANCE,
                8.
            ),
            texture: assets.asteroids.choose(&mut rng).cloned().unwrap(),
            ..Default::default()
        },
        Enemy,
        Hp(rng.gen_range(1..=5)),
        Tumble::new(&mut rng),
        Drops {
            score: 0,
            drops: vec![drop],
        }
    ));
}
