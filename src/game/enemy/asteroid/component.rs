use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use super::MAX_ASTEROID_SPEED;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Tumble {
    pub speed: Vec2,
    pub angular: f32,
}

impl Tumble {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            speed: Vec2::from_angle(rng.gen_range(0.0..PI)) * rng.gen_range(0.0..=MAX_ASTEROID_SPEED),
            angular: rng.gen_range(-PI..=PI),
        }
    }
}
