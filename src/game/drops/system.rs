use bevy::prelude::*;

use crate::game::player::component::Player;
use crate::game::resource::{Experience, Score};
use crate::game::increase_score;
use crate::my_assets::MyAssets;

use super::component::*;
use super::PLAYER_COLLECTION_DISTANCE;

pub fn collect_drops(
    mut commands: Commands,
    drop_query: Query<(Entity, &Transform, &Drop)>,
    player_query: Query<&Transform, (With<Player>, Without<Drop>)>,
    assets: Res<MyAssets>,
    mut experience: ResMut<Experience>,
    mut score: ResMut<Score>,
) {
    let player = match player_query.get_single() {
        Ok(p) => p.translation.xy(),
        _ => return,
    };
    for (entity, transform, drop) in drop_query.iter() {
        if player.distance(transform.translation.xy()) < PLAYER_COLLECTION_DISTANCE {
            commands.entity(entity).despawn();
            match drop {
                Drop::Coin => {
                    increase_score(&mut commands, 200, transform.translation.xy(), &mut score, &assets);
                }
                Drop::Experience(exp) => experience.0 += *exp,
                Drop::Vacuum => {
                    for (e, _, d) in drop_query.iter() {
                        match d {
                            Drop::Experience(_) => {
                                commands.entity(e).try_insert((
                                    Vacuumed(0.),
                                ));
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

pub fn move_vacuumed(
    mut vacuumed_query: Query<(&mut Vacuumed, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Vacuumed>)>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.get_single() {
        let dt = time.delta_seconds();
        for (mut vacuumed, mut transform) in vacuumed_query.iter_mut() {
            vacuumed.0 += dt;
            let speed = vacuumed.0.min(3.) * 500. * dt;
            let mut vector = player.translation.xy() - transform.translation.xy();
            if vector.length() > speed {
                vector = vector.normalize() * speed;
            }
            transform.translation.x += vector.x;
            transform.translation.y += vector.y;
        }
    }
}
