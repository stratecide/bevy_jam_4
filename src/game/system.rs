use std::collections::HashMap;

use bevy::prelude::*;

use super::enemy::resource::EnemyUpgrades;
use super::player::component::Upgrade;
use super::player::resource::Upgrades;
use super::weapon::component::Bullet;
use super::resource::*;
use super::component::*;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1000.),
        ..Default::default()
    });
}

pub fn reset_resources(
    mut commands: Commands
) {
    commands.insert_resource(WaveTimer(0.));
    commands.insert_resource(Score(0));
    commands.insert_resource(Level(1));
    commands.insert_resource(Experience(0));
    commands.insert_resource(AvailableUpgrades(0));
    let mut player_upgrades = HashMap::new();
    player_upgrades.insert(Upgrade::MainBulletCount, 1);
    player_upgrades.insert(Upgrade::ExtraLife, 3);
    commands.insert_resource(Upgrades(player_upgrades));
    commands.insert_resource(EnemyUpgrades(HashMap::new()));
}

pub fn level_up(
    mut level: ResMut<Level>,
    mut exp: ResMut<Experience>,
    mut upgrades: ResMut<AvailableUpgrades>,
) {
    while exp.0 >= level.exp_needed_for_next_level() {
        exp.0 -= level.exp_needed_for_next_level();
        level.0 += 1;
        upgrades.0 += 1;
    }
}

pub fn tick_wave_timer(
    mut wave_timer: ResMut<WaveTimer>,
    timer: Res<Time>,
) {
    wave_timer.0 += timer.delta_seconds();
}

pub fn move_non_bullets(
    mut query: Query<(&mut Transform, &Velocity), Without<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.speed.x * time.delta_seconds();
        transform.translation.y += velocity.speed.y * time.delta_seconds();
    }
}

pub fn fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeAway, Option<&mut Sprite>, Option<&mut Text>)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade_away, sprite, text) in query.iter_mut() {
        fade_away.tick(dt);
        if fade_away.progress() >= 1. {
            commands.entity(entity).despawn();
        } else{
            if let Some(mut sprite) = sprite {
                sprite.color.set_a(1. - fade_away.progress());
            }
            if let Some(mut text) = text {
                for section in &mut text.sections {
                    section.style.color.set_a(1. - fade_away.progress());
                }
            }
        }
    }
}
