use bevy::prelude::*;

use super::component::*;

pub fn despawn_dead(
    mut commands: Commands,
    entity_query: Query<(Entity, &Hp)>,
) {
    for (entity, hp) in entity_query.iter() {
        if hp.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}
