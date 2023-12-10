pub mod asteroid;
pub mod simple;
pub mod boss;
pub mod component;
pub mod resource;
mod system;
use system::*;
use crate::GameState;

use self::component::*;

use bevy::prelude::*;

use super::{despawn, GameSystems, ZOOM};

pub const SPAWN_DISTANCE: f32 = 800. * ZOOM;

#[derive(Debug)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(asteroid::AsteroidPlugin)
        .add_plugins(simple::SimpleEnemyPlugin)
        .add_systems(OnExit(GameState::Game), (
            despawn::<Enemy>,
        ))
        .add_systems(FixedUpdate, update_enemy_velocity
            .in_set(GameSystems::UpdateVelocity))
        .add_systems(FixedUpdate, enemy_collisions
            .in_set(GameSystems::Collision))
        .add_systems(FixedUpdate, despawn_dead
            .in_set(GameSystems::Despawn))
        ;
    }
}