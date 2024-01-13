use bevy::app::App;
use bevy::prelude::{Plugin, Resource, Vec3};

#[derive(Debug, Resource)]
pub struct PhysicRules {
    pub gravity: Vec3,
}

impl Default for PhysicRules {
    fn default() -> Self {
        Self {
            gravity: Vec3 {
                x: 0.0,
                y: -90.8,
                z: 0.0,
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SimulationTrigger {
    Reset,
    AddParticle(u64),
    ChangeParticleScale(f32),
}

#[derive(Debug, Resource)]
pub struct SimulationState {
    pub freeze: bool,
    pub trigger: Vec<SimulationTrigger>,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            freeze: false,
            trigger: vec![],
        }
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>()
            .init_resource::<PhysicRules>();
    }
}
