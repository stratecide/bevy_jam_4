use std::collections::HashMap;

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use super::component::Upgrade;

#[derive(Resource)]
pub struct Upgrades(pub HashMap<Upgrade, usize>);

impl Upgrades {
    pub fn get(&self, key: Upgrade) -> usize {
        self.0.get(&key).cloned().unwrap_or(0)
    }

    pub fn generate_options(&self) -> Vec<Upgrade> {
        let total_upgrades: usize = self.0.values().sum();
        let mut options = vec![
            Upgrade::ExtraLife,
            Upgrade::MovementSpeed,
            Upgrade::MainBulletCount,
            Upgrade::StarBulletCount,
            Upgrade::SpiralBulletCount,
        ];
        if self.0.contains_key(&Upgrade::MainBulletCount) {
            options.push(Upgrade::MainBulletCooldown)
        }
        if self.0.contains_key(&Upgrade::StarBulletCount) {
            options.push(Upgrade::StarBulletCooldown)
        }
        if self.0.contains_key(&Upgrade::SpiralBulletCount) {
            options.push(Upgrade::SpiralBulletCooldown)
        }
        options.shuffle(&mut thread_rng());
        if total_upgrades < 10 && total_upgrades % 2 == 0 {
            // guarantee a bullet-count upgrade
            let weapons = vec![
                Upgrade::MainBulletCount,
                Upgrade::StarBulletCount,
                Upgrade::SpiralBulletCount,
            ];
            if !weapons.contains(&options[0]) && !weapons.contains(&options[1]) {
                options[0] = weapons.choose(&mut thread_rng()).unwrap().clone();
            }
        }
        options.into_iter()
        .filter(|o| o.max().and_then(|m| Some(self.get(*o) < m)).unwrap_or(true))
        .take(2)
        .collect()
    }
}
