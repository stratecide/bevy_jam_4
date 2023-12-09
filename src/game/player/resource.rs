use std::collections::HashMap;

use bevy::prelude::*;

use super::component::Upgrade;

#[derive(Resource)]
pub struct Upgrades(pub HashMap<Upgrade, usize>);

impl Upgrades {
    pub fn get(&self, key: Upgrade) -> usize {
        self.0.get(&key).cloned().unwrap_or(0)
    }

    pub fn generate_options(&self) -> Vec<Upgrade> {
        // TODO
        vec![
            Upgrade::MainBulletCooldown,
            Upgrade::MainBulletCount,
        ]
    }
}
