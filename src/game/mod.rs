pub mod player;
pub mod weapon;
pub mod enemy;
pub mod drops;
pub mod ui;
mod system;
use system::*;
pub mod component;
pub mod resource;

use bevy::prelude::*;

use crate::GameState;

pub const ZOOM: f32 = 1.5;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(weapon::WeaponPlugin)
        .add_plugins(drops::DropsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_state::<PauseState>()
        .add_systems(OnEnter(GameState::Game), (
            spawn_camera,
            reset_resources,
        ))
        .configure_sets(FixedUpdate, (
            GameSystems::UpdateVelocity,
            GameSystems::ShipMovement,
            GameSystems::Weapon,
            GameSystems::BulletMovement,
            GameSystems::Despawn,
            GameSystems::Collision,
            GameSystems::SpawnEnemy,
        ).chain().run_if(in_state(GameState::Game).and_then(in_state(PauseState::Unpaused))))
        .add_systems(FixedUpdate, move_non_bullets.in_set(GameSystems::ShipMovement))
        .add_systems(FixedUpdate, fade_out.before(GameSystems::Collision))
        .add_systems(FixedUpdate, level_up.run_if(in_state(GameState::Game).and_then(in_state(PauseState::Unpaused))))
        ;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
    Shop,
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
