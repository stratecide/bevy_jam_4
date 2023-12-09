pub mod component;
pub mod spawn;
mod system;
use system::*;

use bevy::prelude::*;

use crate::game::GameSystems;

#[derive(Debug)]
pub struct SimpleEnemyPlugin;

impl Plugin for SimpleEnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_systems(FixedUpdate, spawn_small_enemy
        //    .in_set(GameSystems::SpawnEnemy))
        ;
    }
}