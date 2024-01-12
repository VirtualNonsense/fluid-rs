use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::particle::{Particle, spawn_particle};
use crate::resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_simulation_trigger);
        app.add_systems(Update, constrain_particle_in_window);
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
pub fn constrain_particle_in_window(
    mut entity_query: Query<(&mut Transform, &mut Particle)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // dbg!("updating constraint");
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    for (mut trans, mut entity) in entity_query.iter_mut() {

        // dbg!(&entity.velocity);
        let lower_x = trans.translation.x - entity.radius;
        let upper_x = trans.translation.x + entity.radius;
        let lower_y = trans.translation.y - entity.radius;
        let upper_y = trans.translation.y + entity.radius;
        if upper_y > height {
            entity.velocity.y = -1. * entity.velocity.y.abs() * entity.dampening;
            trans.translation.y = height - entity.radius;
        } else if lower_y < 0.0 {
            entity.velocity.y = entity.velocity.y.abs() * entity.dampening;
            trans.translation.y = entity.radius;
        }
        if upper_x > width {
            entity.velocity.x = entity.velocity.x.abs() * -1. * entity.dampening;
            trans.translation.x = width - entity.radius;
        } else if lower_x < 0.0 {
            trans.translation.x = entity.radius;
            entity.velocity.x = entity.velocity.x.abs() * entity.dampening;
        }
    }
}