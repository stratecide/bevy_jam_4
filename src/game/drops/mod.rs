pub mod component;
mod system;
use system::*;

use bevy::prelude::*;

use crate::game::GameSystems;

pub const PLAYER_COLLECTION_DISTANCE: f32 = 50.;

#[derive(Debug)]
pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, collect_drops
            .in_set(GameSystems::Collision))
        ;
    }
}