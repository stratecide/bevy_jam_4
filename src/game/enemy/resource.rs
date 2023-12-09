use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::drops::component::Drop;
use crate::game::player::component::Upgrade;
use crate::my_assets::MyAssets;

use super::simple::spawn::*;
use super::asteroid::*;

#[derive(Resource)]
pub struct EnemyUpgrades(pub HashMap<Upgrade, usize>);

pub type WaveSpawner = dyn Fn(usize, &mut Commands, Vec2, &MyAssets) + Send + Sync + 'static;

#[derive(Resource)]
pub struct Waves {
    pub total_duration: f32,
    waves: Vec<(f32, Box<WaveSpawner>)>,
}

impl Waves {
    fn new() -> Self {
        Self {
            total_duration: 0.,
            waves: Vec::new(),
        }
    }

    pub fn wave(mut self, seconds: f32, method: impl Fn(usize, &mut Commands, Vec2, &MyAssets) + Send + Sync + 'static) -> Self {
        self.add_wave(seconds, method);
        self
    }

    pub fn add_wave(&mut self, seconds: f32, method: impl Fn(usize, &mut Commands, Vec2, &MyAssets) + Send + Sync + 'static) {
        self.waves.push((self.total_duration, Box::new(method)));
        self.total_duration += seconds;
    }

    pub fn tick(&self, time_before: f32, time_after: f32, commands: &mut Commands, player_pos: Vec2, assets: &MyAssets) {
        let mut difficulty_before = (time_before / self.total_duration) as usize;
        let difficulty_after = (time_after / self.total_duration) as usize;
        let mut time_before = time_before - self.total_duration * difficulty_before as f32;
        let time_after = time_after - self.total_duration * difficulty_after as f32;
        let mut next = self.waves.len();
        for i in 0..self.waves.len() {
            if self.waves[i].0 >= time_before {
                next = i;
                break;
            }
        }
        if next == self.waves.len() {
            next = 0;
            time_before = 0.;
            difficulty_before += 1;
        }
        while difficulty_before <= difficulty_after {
            if self.waves[next].0 < time_after || difficulty_before < difficulty_after {
                (self.waves[next].1)(difficulty_before, commands, player_pos, assets);
                time_before += self.waves[next].0;
                next += 1;
                if next == self.waves.len() {
                    next = 0;
                    time_before = 0.;
                    difficulty_before += 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn default() -> Self {
        let mut result = Self::new()
        .wave(0., |_difficulty, commands, center, assets| {
            spawn_debris(commands, center, assets, Drop::Coin, 1);
        });
        for _ in 0..10 {
            result.add_wave(4., |difficulty, commands, center, assets| {
                spawn_red4_ring(commands, center, assets, difficulty, 2 + difficulty, None);
            });
        }
        result.add_wave(12., |difficulty, commands, center, assets| {
            spawn_red4_ring(commands, center, assets, difficulty, 6 + difficulty * 2, Some(Drop::Vacuum));
        });

        result.add_wave(0., |_difficulty, commands, center, assets| {
            spawn_debris(commands, center, assets, Drop::Coin, 2);
        });
        for _ in 0..10 {
            result.add_wave(4., |difficulty, commands, center, assets| {
                spawn_red1_squad(commands, center, assets, difficulty, 1, None);
            });
        }

        result.add_wave(0., |_difficulty, commands, center, assets| {
            spawn_debris(commands, center, assets, Drop::Coin, 1);
        });
        result.add_wave(12., |difficulty, commands, center, assets| {
            spawn_red1_squad(commands, center, assets, difficulty, 3 + difficulty, None);
        });
        for _ in 0..10 {
            result.add_wave(1., |difficulty, commands, center, assets| {
                spawn_red4_squad(commands, center, assets, difficulty, 0, None);
            });
            result.add_wave(1., |difficulty, commands, center, assets| {
                spawn_red1_squad(commands, center, assets, difficulty, 0, None);
            });
        }

        result.add_wave(4., |_difficulty, commands, center, assets| {
            spawn_debris(commands, center, assets, Drop::Vacuum, 0);
        });
        result
    }
}
