use bevy::prelude::*;

#[derive(Component)]
pub struct Drops {
    pub experience: usize,
    pub score: usize,
}

#[derive(Component)]
pub enum Drop {
    Experience(usize),
}

