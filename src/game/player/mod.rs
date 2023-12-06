mod system;
use system::*;
pub mod component;

use bevy::prelude::*;

use crate::GameState;

use self::component::*;

use super::{GameSystems, despawn};

pub const PLAYER_SPEED: f32 = 300.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Game), (
            spawn_player,
        ))
        .add_systems(OnExit(GameState::Game), (
            despawn::<Player>,
        ))
        .add_systems(FixedUpdate, (
            player_input,
        ).in_set(GameSystems::ShipMovement))
        ;
    }
}
