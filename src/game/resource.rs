use bevy::prelude::*;

#[derive(Resource)]
pub struct Experience(pub usize);

#[derive(Resource)]
pub struct Level(pub usize);

impl Level {
    pub fn exp_needed_for_next_level(&self) -> usize {
        self.0 * 10
    }
}

#[derive(Resource)]
pub struct AvailableUpgrades(pub usize);
