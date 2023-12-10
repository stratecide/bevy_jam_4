use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Hp(pub usize);

#[derive(Component)]
pub enum MovementPattern {
    StraightApproach(StraightApproach),
    KeepDistance(KeepDistance),
    Hover(Hover),
}

pub struct StraightApproach {
    pub turn_speed: f32,
    pub turnaround_distance: f32,
}

pub struct KeepDistance {
    pub max_speed: f32,
    pub target_distance: f32,
}

pub struct Hover {
    pub target_distance: f32,
    pub angular_speed: f32,
}
