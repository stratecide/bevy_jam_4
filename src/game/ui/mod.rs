mod system;
use system::*;
pub mod component;

use bevy::prelude::*;

use crate::GameState;

use super::{GameSystems, PauseState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Game), (
            setup_ui,
        ))
        .add_systems(FixedUpdate, (
            update_expbar,
            update_life_counter,
            update_wave_counter,
            update_score_counter,
        ).in_set(GameSystems::SpawnEnemy))
        .add_systems(Update, open_shop.run_if(in_state(GameState::Game).and_then(in_state(PauseState::Unpaused))))
        .add_systems(Update, update_shop.run_if(in_state(GameState::Game).and_then(in_state(PauseState::Shop))))
        .add_systems(OnExit(GameState::Game), (
            delete_ui,
        ))
        ;
    }
}
