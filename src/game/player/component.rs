
use bevy::prelude::*;

pub const MOVEMENT_SPEED_BONUS: f32 = 0.2;

#[derive(Component)]
pub struct Player {

}

#[derive(Component)]
pub struct PlayerFriend;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Upgrade {
    MovementSpeed,
    MainBulletCount,
}

impl Upgrade {
    pub fn title(&self) -> &'static str {
        match self {
            Self::MovementSpeed => "Speed +",
            Self::MainBulletCount => "Main Bullets",
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::MovementSpeed => format!("Increases your movement speed by {}% (additive)", (MOVEMENT_SPEED_BONUS * 100.).round()),
            Self::MainBulletCount => format!("You shoot one additional bullet when you shoot"),
        }
    }
}
