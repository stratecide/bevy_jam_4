use bevy::prelude::*;

use crate::game::player::component::Player;
use crate::game::resource::{Experience, Score};
use crate::game::increase_score;
use crate::my_assets::MyAssets;

use super::{component::*, PLAYER_COLLECTION_DISTANCE};

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
                    increase_score(&mut commands, 1000, transform.translation.xy(), &mut score, &assets);
                }
                Drop::Experience(exp) => experience.0 += *exp,
            }
        }
    }
}
