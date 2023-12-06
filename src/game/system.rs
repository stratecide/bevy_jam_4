use bevy::prelude::*;

use super::{component::{FadeAway, Velocity}, weapon::component::Bullet};

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1000.),
        ..Default::default()
    });
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
    mut query: Query<(Entity, &mut FadeAway, Option<&mut Sprite>)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade_away, sprite) in query.iter_mut() {
        fade_away.tick(dt);
        if fade_away.progress() >= 1. {
            commands.entity(entity).despawn();
        } else if let Some(mut sprite) = sprite {
            sprite.color.set_a(1. - fade_away.progress());
        }
    }
}
