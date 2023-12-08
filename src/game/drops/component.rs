use bevy::prelude::*;

#[derive(Component)]
pub struct DropsExperience(pub usize);

#[derive(Component)]
pub enum Drop {
    Experience(usize),
}

