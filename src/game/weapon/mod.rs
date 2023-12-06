pub mod component;
use component::*;
mod system;
use system::*;

use bevy::prelude::*;

use crate::GameState;

use super::{GameSystems, despawn};

#[derive(Debug)]
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, (
            tick_weapons::<MainCannon>,
        ).in_set(GameSystems::Weapon))
        .add_systems(FixedUpdate, tick_bullets
            .in_set(GameSystems::BulletMovement))
        .add_systems(FixedUpdate, despawn_offscreen
            .in_set(GameSystems::Despawn))
        .add_systems(OnExit(GameState::Game), (
            despawn::<Bullet>,
        ))
        ;
    }
}
