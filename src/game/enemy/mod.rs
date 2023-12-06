pub mod asteroid;
pub mod component;
mod system;
use system::*;
use crate::GameState;

use self::component::*;

use bevy::prelude::*;

use super::{despawn, GameSystems};


#[derive(Debug)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(asteroid::AsteroidPlugin)
        .add_systems(OnExit(GameState::Game), (
            despawn::<Enemy>,
        ))
        .add_systems(FixedUpdate, despawn_dead.in_set(GameSystems::Despawn))
        ;
    }
}