use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

use crate::game::enemy::component::{Enemy, Hp};
use crate::my_assets::MyAssets;

use super::component::*;

pub fn spawn_asteroids(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    assets: Res<MyAssets>,
) {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(c) => c,
        _ => return,
    };
    let camera_size = match camera.logical_viewport_size() {
        Some(s) => s,
        _ => return,
    };
    let camera_translation = camera_transform.translation().xy();
    let mut rng = thread_rng();
    if rng.gen_bool(0.01) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen_range(0.0..=camera_size.x) - camera_size.x / 2. + camera_translation.x,
                    rng.gen_range(0.0..=camera_size.y) - camera_size.y / 2. + camera_translation.y,
                    0.
                ),
                texture: assets.asteroids.choose(&mut rng).cloned().unwrap(),
                ..Default::default()
            },
            Enemy,
            Hp(rng.gen_range(1..=5)),
            Tumble::new(&mut rng),
        ));
    }
}

pub fn move_asteroids(
    mut asteroid_query: Query<(&mut Transform, &Tumble)>,
    time: Res<Time>,
) {
    for (mut transform, tumble) in asteroid_query.iter_mut() {
        transform.rotate_axis(Vec3::Z, tumble.angular * time.delta_seconds());
        transform.translation.x += tumble.speed.x * time.delta_seconds();
        transform.translation.y += tumble.speed.y * time.delta_seconds();
    }
}
