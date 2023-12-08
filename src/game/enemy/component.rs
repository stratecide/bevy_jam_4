use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Hp(pub usize);

#[derive(Component)]
pub enum MovementPattern {
    StraightApproach(StraightApproach),
}

pub struct StraightApproach {
    pub turn_speed: f32,
    pub turnaround_distance: f32,
}
