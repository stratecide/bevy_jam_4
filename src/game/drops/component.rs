use bevy::prelude::*;

use crate::my_assets::MyAssets;

#[derive(Component)]
pub struct Drops {
    pub score: usize,
    pub drops: Vec<Drop>,
}

#[derive(Clone, Copy, Component)]
pub enum Drop {
    Coin,
    Experience(usize),
    Vacuum,
}

impl Drop {
    pub fn sprite(&self, pos: Vec2, assets: &MyAssets) -> SpriteBundle {
        let transform = Transform::from_xyz(pos.x, pos.y, 0.).with_scale(Vec3::splat(match self {
            Self::Coin => 1.5,
            Self::Experience(exp) => ((*exp as f32).sqrt() + 1.).ln(),
            Self::Vacuum => 1.5,
        }));
        let texture = match self {
            Self::Coin => assets.coin.clone(),
            Self::Experience(_) => assets.exp.clone(),
            Self::Vacuum => assets.vacuum.clone(),
        };
        SpriteBundle {
            transform,
            texture,
            ..Default::default()
        }
    }
}

#[derive(Component)]
pub struct Vacuumed(pub f32);
