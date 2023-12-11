use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct HighScores(pub Vec<usize>);

#[derive(Default, Resource)]
pub struct CurrentScoreIndex(pub Option<usize>);
