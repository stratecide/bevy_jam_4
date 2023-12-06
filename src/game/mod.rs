pub mod player;
pub mod weapon;
mod system;
use system::*;

use bevy::prelude::*;

use crate::GameState;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(player::PlayerPlugin)
        .add_plugins(weapon::WeaponPlugin)
        .add_systems(OnEnter(GameState::Game), (
            spawn_camera,
        ))
        .configure_sets(FixedUpdate, (
            GameSystems::ShipMovement,
            GameSystems::Weapon,
            GameSystems::BulletMovement,
            GameSystems::Despawn,
            GameSystems::Collision,
        ).chain().run_if(in_state(GameState::Game)))
        ;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystems {
    ShipMovement,
    Weapon,
    BulletMovement,
    Despawn,
    Collision,
}

pub fn despawn<T: Component>(
    mut commands: Commands,
    entity_query: Query<Entity, With<T>>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
