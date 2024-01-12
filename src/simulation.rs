use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::entity::spawn_particle;
use crate::resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_simulation_trigger);
    }
}

fn handle_simulation_trigger(
    mut sim_state: ResMut<SimulationState>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut particle_count: u64 = 0;
    for trig in &sim_state.trigger {
        match trig {
            SimulationTrigger::Reset => {
                println!("resetting simulation");
            }
            SimulationTrigger::AddParticle => {
                println!("adding a new particle");
                particle_count += 1;
            }
        }
    }
    if particle_count > 0 {
        spawn_particle(
            commands,
            window_query,
            meshes,
            materials,
            particle_count
        );
    }
    let len = sim_state.trigger.len();
    sim_state.trigger.drain(0..len);
}