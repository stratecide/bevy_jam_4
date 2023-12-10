
use bevy::prelude::*;

pub const MOVEMENT_SPEED_BONUS: f32 = 0.2;
pub const MAIN_WEAPON_COOLDOWN_REDUCTION: f32 = 0.2;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerFriend;

#[derive(Component)]
pub struct PlayerMovement;

#[derive(Component)]
pub struct PlayerSatellite(pub f32);

#[derive(Component)]
pub struct Vulnerability(f32);

impl Vulnerability {
    pub fn new() -> Self {
        Self(0.)
    }
    pub fn tick(&mut self, delta_seconds: f32) {
        self.0 = (self.0 - delta_seconds).max(0.)
    }
    pub fn reset(&mut self) {
        self.0 = 1.
    }
    pub fn remaining_seconds(&self) -> f32 {
        self.0
    }
    pub fn vulnerable(&self) -> bool {
        self.0 <= 0.
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Upgrade {
    ExtraLife,
    MovementSpeed,
    MainBulletCount,
    MainBulletCooldown,
    StarBulletCount,
    StarBulletCooldown,
    SpiralBulletCount,
    SpiralBulletCooldown,
}

impl Upgrade {
    pub fn unlock_title(&self) -> Option<&'static str> {
        Some(match self {
            Self::MainBulletCount => "Unlock Main Cannon",
            Self::StarBulletCount => "Unlock Star Cannon",
            Self::SpiralBulletCount => "Unlock Spiral Cannon",
            _ => return None,
        })
    }


    pub fn title(&self) -> &'static str {
        match self {
            Self::ExtraLife => "Extra Life",
            Self::MovementSpeed => "Speed +",
            Self::MainBulletCount => "Extra Bullet (Main)",
            Self::MainBulletCooldown => "Weapon Cooldown (Main)",
            Self::StarBulletCount => "Extra Bullet (Star)",
            Self::StarBulletCooldown => "Weapon Cooldown (Star)",
            Self::SpiralBulletCount => "Extra Bullet (Spiral)",
            Self::SpiralBulletCooldown => "Weapon Cooldown (Spiral)",
        }
    }

    pub fn unlock_description(&self) -> Option<String> {
        Some(match self {
            Self::MainBulletCount => "Shoots 1 bullet toward the mouse cursor",
            Self::StarBulletCount => "Shoots 2 bullets in random directions",
            Self::SpiralBulletCount => "A UFO will circle you, shooting bullets in a spiral pattern",
            _ => return None,
        }.to_string())
    }

    pub fn description(&self) -> String {
        match self {
            Self::ExtraLife => format!("You will survive one hit"),
            Self::MovementSpeed => format!("Increases your movement speed by {}% (additive)", (MOVEMENT_SPEED_BONUS * 100.).round()),
            Self::MainBulletCount => format!("You shoot one additional bullet when you shoot using your main weapon"),
            Self::MainBulletCooldown => format!("Reduces the cooldown of your main weapon by {}% (multiplicative)", (MAIN_WEAPON_COOLDOWN_REDUCTION * 100.).round()),
            Self::StarBulletCount => format!("You shoot one additional bullet when you shoot using your Star weapon"),
            Self::StarBulletCooldown => format!("Reduces the cooldown of your Star weapon by {}% (multiplicative)", (MAIN_WEAPON_COOLDOWN_REDUCTION * 100.).round()),
            Self::SpiralBulletCount => format!("You shoot one additional bullet when you shoot using your Spiral weapon"),
            Self::SpiralBulletCooldown => format!("Reduces the cooldown of your Spiral weapon by {}% (multiplicative)", (MAIN_WEAPON_COOLDOWN_REDUCTION * 100.).round()),
        }
    }

    pub fn max(&self) -> Option<usize> {
        match self {
            Self::MovementSpeed => Some(5),
            Self::ExtraLife => Some(9),
            _ => Some(8),
        }
    }
}
