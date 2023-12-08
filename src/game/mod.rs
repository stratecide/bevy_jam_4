pub mod player;
pub mod weapon;
pub mod enemy;
mod system;
use system::*;
pub mod component;

use bevy::prelude::*;

use crate::GameState;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(weapon::WeaponPlugin)
        .add_systems(OnEnter(GameState::Game), (
            spawn_camera,
        ))
        .configure_sets(FixedUpdate, (
            GameSystems::UpdateVelocity,
            GameSystems::ShipMovement,
            GameSystems::Weapon,
            GameSystems::BulletMovement,
            GameSystems::Despawn,
            GameSystems::Collision,
            GameSystems::SpawnEnemy,
        ).chain().run_if(in_state(GameState::Game)))
        .add_systems(FixedUpdate, move_non_bullets.in_set(GameSystems::ShipMovement))
        .add_systems(FixedUpdate, fade_out.before(GameSystems::Collision))
        ;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystems {
    UpdateVelocity,
    ShipMovement,
    Weapon,
    BulletMovement,
    Despawn,
    Collision,
    SpawnEnemy,
}

pub fn despawn<T: Component>(
    mut commands: Commands,
    entity_query: Query<Entity, With<T>>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
