use std::marker::PhantomData;
use bevy::prelude::*;
use bevy::audio::*;

use crate::game::component::Velocity;
use crate::my_assets::MyAssets;
use crate::game::player::component::PlayerFriend;

pub trait Weapon: Component {
    fn max_cooldown(&self) -> f32;
    fn fire(&self, commands: &mut Commands, entity_transform: &Transform, friendly: bool, assets: &Res<MyAssets>);
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
    fn max_cooldown(&self) -> f32 {
        self.cooldown
    }
    
    fn fire(&self, commands: &mut Commands, entity_transform: &Transform, friendly: bool, assets: &Res<MyAssets>) {
        let forward: Vec2 = entity_transform.local_y().xy();
        let sideways: Vec2 = Vec2::new(forward.y, -forward.x);
        let (texture, bullet_speed) = if friendly {
            (&assets.player_bullet, 1000.)
        } else {
            (&assets.enemy_bullet, 300.)
        };
        for i in 0..self.bullets {
            let mut pos = entity_transform.translation.xy() + forward * 35. * entity_transform.scale.y;
            pos += ((i * 2 + 1) as f32 - self.bullets as f32) * sideways * 5.;
            let mut transform = Transform::from_xyz(pos.x, pos.y, entity_transform.translation.z - 1.);
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
                volume: Volume::Relative(VolumeLevel::new((self.bullets as f32 + 1.).ln())),
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
