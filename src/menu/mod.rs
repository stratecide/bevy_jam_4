mod system;
use system::*;
pub mod component;
pub mod resource;
use resource::*;

use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<HighScores>()
        .init_resource::<CurrentScoreIndex>()
        .add_systems(OnEnter(GameState::Menu), (
            setup_ui,
        ))
        .add_systems(Update, (
            start_game,
        ).run_if(in_state(GameState::Menu)))
        ;
    }
}
