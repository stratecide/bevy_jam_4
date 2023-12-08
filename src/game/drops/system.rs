use bevy::prelude::*;

use crate::game::{player::component::Player, resource::Experience};

use super::{component::*, PLAYER_COLLECTION_DISTANCE};

pub fn collect_drops(
    mut commands: Commands,
    drop_query: Query<(Entity, &Transform, &Drop)>,
    player_query: Query<&Transform, (With<Player>, Without<Drop>)>,
    mut experience: ResMut<Experience>,
) {
    let player = match player_query.get_single() {
        Ok(p) => p.translation.xy(),
        _ => return,
    };
    for (entity, transform, drop) in drop_query.iter() {
        if player.distance(transform.translation.xy()) < PLAYER_COLLECTION_DISTANCE {
            commands.entity(entity).despawn();
            match drop {
                Drop::Experience(exp) => experience.0 += *exp,
            }
        }
    }
}
