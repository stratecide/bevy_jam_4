mod system;
use system::*;
pub mod component;

use bevy::prelude::*;

use crate::GameState;

use super::GameSystems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Game), (
            setup_ui,
        ))
        .add_systems(FixedUpdate, update_exp.in_set(GameSystems::SpawnEnemy))
        /*.add_systems(OnExit(GameState::Game), (
            despawn::<Player>,
        ))*/
        ;
    }
}
