use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::player::component::Upgrade;

#[derive(Resource)]
pub struct EnemyUpgrades(pub HashMap<Upgrade, usize>);
