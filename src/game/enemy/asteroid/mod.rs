pub mod component;
mod system;
use system::*;

use bevy::prelude::*;

use crate::game::{GameSystems, player::PLAYER_SPEED};

pub const MAX_ASTEROID_SPEED: f32 = PLAYER_SPEED * 0.5;

#[derive(Debug)]
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, spawn_asteroids
            .in_set(GameSystems::SpawnEnemy))
        .add_systems(FixedUpdate, move_asteroids
            .in_set(GameSystems::ShipMovement))
        ;
    }
}