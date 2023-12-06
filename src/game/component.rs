use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub speed: Vec2,
}

#[derive(Component)]
pub struct FadeAway {
    duration: f32,
    current: f32,
}

impl FadeAway {
    pub fn new(seconds: f32) -> Self {
        Self {
            duration: seconds,
            current: 0.,
        }
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.current += delta_seconds;
    }

    pub fn progress(&self) -> f32 {
        self.current / self.duration
    }
}

