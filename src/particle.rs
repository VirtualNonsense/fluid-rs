use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Particle {
    pub mass: f32,
    pub bounciness: f32,
    pub radius: f32,
    pub outer_radius: f32,
}