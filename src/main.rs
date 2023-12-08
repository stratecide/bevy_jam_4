#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod game;
mod my_assets;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
}

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        // state
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Game))
        .add_collection_to_loading_state::<_, my_assets::MyAssets>(GameState::Loading)
        .run();
}
