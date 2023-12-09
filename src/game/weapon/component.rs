use std::collections::HashMap;
use std::f32::consts::PI;
use std::marker::PhantomData;
use bevy::prelude::*;
use bevy::audio::*;

use crate::game::component::Velocity;
use crate::game::player::component::MAIN_WEAPON_COOLDOWN_REDUCTION;
use crate::game::player::component::Upgrade;
use crate::game::resource::WaveTimer;
use crate::my_assets::MyAssets;
use crate::game::player::component::PlayerFriend;

pub trait Weapon: Component {
    fn max_cooldown(&self, upgrades: &HashMap<Upgrade, usize>) -> f32;
    fn fire(&self, commands: &mut Commands, entity_transform: &Transform, upgrades: &HashMap<Upgrade, usize>, friendly: bool, assets: &MyAssets, time: &WaveTimer);
}

#[derive(Component)]
pub struct MainCannon {
    pub bullets: usize,
    cooldown: f32,
}

impl MainCannon {
    pub fn new(bullets: usize, cooldown: f32) -> WeaponBundle<Self> {
        WeaponBundle {
            weapon: Self {
                bullets,
                cooldown,
            },
            cooldown: WeaponCooldown {
                cooldown,
                _p: PhantomData,
            },
        }
    }
}

impl Weapon for MainCannon {
    fn max_cooldown(&self, upgrades: &HashMap<Upgrade, usize>) -> f32 {
        self.cooldown * (1. - MAIN_WEAPON_COOLDOWN_REDUCTION).powi(upgrades.get(&Upgrade::MainBulletCooldown).cloned().unwrap_or(0) as i32)
    }
    
    fn fire(&self, commands: &mut Commands, entity_transform: &Transform, upgrades: &HashMap<Upgrade, usize>, friendly: bool, assets: &MyAssets, _: &WaveTimer) {
        let forward: Vec2 = entity_transform.local_y().xy();
        let sideways: Vec2 = Vec2::new(forward.y, -forward.x);
        let (texture, bullet_speed) = if friendly {
            (&assets.player_bullet, 1000.)
        } else {
            (&assets.enemy_bullet, 300.)
        };
        let bullet_count = self.bullets + upgrades.get(&Upgrade::MainBulletCount).cloned().unwrap_or(0);
        for i in 0..bullet_count {
            let mut pos = entity_transform.translation.xy() + forward * 35. * entity_transform.scale.y;
            pos += ((i * 2 + 1) as f32 - bullet_count as f32) * sideways * 5.;
            let mut transform = Transform::from_xyz(pos.x, pos.y, 30.);
            transform.rotation = entity_transform.rotation;
            let mut bundle = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(0., 0.3)),
                        ..Default::default()
                    },
                    transform,
                    texture: texture.clone(),
                    ..Default::default()
                },
                Bullet::default(),
                Velocity {
                    speed: forward * bullet_speed,
                },
            ));
            if friendly {
                bundle.insert(PlayerFriend);
            }
        }
        commands.spawn(AudioBundle {
            source: assets.shooting.clone(),
            settings: PlaybackSettings {
                // get louder the more bullets are shot at once ..?
                volume: Volume::Relative(VolumeLevel::new(1.)),
                ..Default::default()
            }
        });
    }
}

#[derive(Component)]
pub struct StarCannon {
    pub bullets: usize,
    cooldown: f32,
}

impl StarCannon {
    pub fn new(bullets: usize, cooldown: f32) -> WeaponBundle<Self> {
        WeaponBundle {
            weapon: Self {
                bullets,
                cooldown,
            },
            cooldown: WeaponCooldown {
                cooldown,
                _p: PhantomData,
            },
        }
    }
}

impl Weapon for StarCannon {
    fn max_cooldown(&self, upgrades: &HashMap<Upgrade, usize>) -> f32 {
        self.cooldown * (1. - MAIN_WEAPON_COOLDOWN_REDUCTION).powi(upgrades.get(&Upgrade::StarBulletCooldown).cloned().unwrap_or(0) as i32)
    }
    
    fn fire(&self, commands: &mut Commands, entity_transform: &Transform, upgrades: &HashMap<Upgrade, usize>, friendly: bool, assets: &MyAssets, time: &WaveTimer) {
        let angle = time.0 / 10.;
        let (texture, bullet_speed) = if friendly {
            (&assets.player_bullet, 1000.)
        } else {
            (&assets.enemy_bullet, 300.)
        };
        let bullet_count = self.bullets + upgrades.get(&Upgrade::StarBulletCount).cloned().unwrap_or(0);
        for i in 0..bullet_count {
            let angle = angle + i as f32 * 2. * PI / bullet_count as f32;
            let forward = Vec2::from_angle(angle);
            let pos = entity_transform.translation.xy() + forward * 40. * entity_transform.scale.y;
            let mut transform = Transform::from_xyz(pos.x, pos.y, 30.);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
            let mut bundle = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(0., 0.3)),
                        ..Default::default()
                    },
                    transform,
                    texture: texture.clone(),
                    ..Default::default()
                },
                Bullet::default(),
                Velocity {
                    speed: forward * bullet_speed,
                },
            ));
            if friendly {
                bundle.insert(PlayerFriend);
            }
        }
        commands.spawn(AudioBundle {
            source: assets.shooting.clone(),
            settings: PlaybackSettings {
                // get louder the more bullets are shot at once ..?
                volume: Volume::Relative(VolumeLevel::new(1.)),
                ..Default::default()
            }
        });
    }
}

#[derive(Component, Default)]
pub struct Bullet {
    pub despawn_timer: f32,
}

#[derive(Component)]
pub struct WeaponCooldown<W: Weapon> {
    pub cooldown: f32,
    _p: PhantomData<W>,
}

#[derive(Bundle)]
pub struct WeaponBundle<W: Weapon> {
    weapon: W,
    cooldown: WeaponCooldown<W>,
}
