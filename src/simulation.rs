use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::particle::{Particle, spawn_particle, VelocityEntity};
use crate::resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_simulation_trigger);
        app.add_systems(Update, constrain_particle_in_window);
    }
}

fn handle_simulation_trigger(
    particle: Res<Particle>,
    mut sim_state: ResMut<SimulationState>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Transform, &VelocityEntity, &Handle<ColorMaterial>)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut particle_count: u64 = 0;
    let mut delete = false;
    for trig in &sim_state.trigger {
        match trig {
            SimulationTrigger::Reset => {
                println!("resetting simulation");
                delete = true;
            }
            SimulationTrigger::AddParticle => {
                println!("adding a new particle");
                particle_count += 1;
            }
            SimulationTrigger::ChangeParticleScale(new_r) => {
                for (_entity, mut t, vel, _co) in query.iter_mut() {
                    t.scale.x = new_r / vel.original_radius;
                    t.scale.y = new_r / vel.original_radius;
                }
            }
        }
    }
    if delete {
        for (entity, _t, _vel, _co) in query.iter() {
            commands.entity(entity).remove::<VelocityEntity>();
            commands.entity(entity).remove::<Handle<ColorMaterial>>();
        }
    }

    if particle_count > 0 {
        spawn_particle(
            commands,
            window_query,
            meshes,
            materials,
            particle_count,
            particle.radius,
        );
    }
    let len = sim_state.trigger.len();
    sim_state.trigger.drain(0..len);
}

pub fn constrain_particle_in_window(
    particle: Res<Particle>,
    mut entity_query: Query<(&mut Transform, &mut VelocityEntity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // dbg!("updating constraint");
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    let radius = particle.radius;
    let dampening = particle.dampening;
    for (mut trans, mut entity) in entity_query.iter_mut() {

        // dbg!(&entity.velocity);

        let lower_x = trans.translation.x - radius;
        let upper_x = trans.translation.x + radius;
        let lower_y = trans.translation.y - radius;
        let upper_y = trans.translation.y + radius;
        if upper_y > height {
            entity.velocity.y = -1. * entity.velocity.y.abs() * dampening;
            trans.translation.y = height - radius;
        } else if lower_y < 0.0 {
            entity.velocity.y = entity.velocity.y.abs() * dampening;
            trans.translation.y = radius;
        }
        if upper_x > width {
            entity.velocity.x = entity.velocity.x.abs() * -1. * dampening;
            trans.translation.x = width - radius;
        } else if lower_x < 0.0 {
            trans.translation.x = radius;
            entity.velocity.x = entity.velocity.x.abs() * dampening;
        }
    }
}