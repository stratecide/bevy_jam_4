pub mod component;
mod system;
use system::*;

use bevy::prelude::*;

use crate::{game::GameSystems, GameState};

use self::component::Drop;

use super::despawn;

pub const PLAYER_COLLECTION_DISTANCE: f32 = 50.;

#[derive(Debug)]
pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, collect_drops
            .in_set(GameSystems::Collision))
        .add_systems(FixedUpdate, move_vacuumed
            .in_set(GameSystems::BulletMovement))
        .add_systems(OnExit(GameState::Game), (
            despawn::<Drop>,
        ))
        ;
    }
}