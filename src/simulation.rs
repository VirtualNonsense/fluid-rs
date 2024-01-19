use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::particle::{Particle, spawn_particle, ParticleEntity};
use crate::resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_simulation_commands);
        app.add_systems(Update, constrain_particle_in_window);
    }
}

fn handle_simulation_commands(
    mut commands: Commands,
    mut sim_state: ResMut<SimulationState>,
    mut particle: ResMut<Particle>,
    mut physic_rules: ResMut<PhysicRules>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Transform, &ParticleEntity, &Handle<ColorMaterial>)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut particle_count: u64 = 0;
    let mut delete = false;
    for command in &sim_state.commands {
        match command {
            SimulationCommands::DeleteAllParticles => {
                println!("deleting all particle");
                delete = true;
            }
            SimulationCommands::AddParticle(amount) => {
                if *amount > 1 {
                    println!("adding {} new particle!", amount);
                } else {
                    println!("adding a new particle!");
                }
                particle_count += *amount;
            }
            SimulationCommands::ChangeParticleScale(new_r) => {
                for (_entity, mut t, vel, _co) in query.iter_mut() {
                    t.scale.x = new_r / vel.original_radius;
                    t.scale.y = new_r / vel.original_radius;

                }
                particle.radius = *new_r;
            }
            SimulationCommands::ChangeGravity(grav) => {
                physic_rules.gravity = *grav;
            }
            SimulationCommands::ChangeParticleMass(mass) => {
                particle.mass = *mass;
            }
            SimulationCommands::ChangeParticleDampening(damp) => {
                particle.dampening = *damp;
            }
        }
    }
    if delete {
        for (entity, _t, _vel, _co) in query.iter() {
            commands.entity(entity).remove::<ParticleEntity>();
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
    let len = sim_state.commands.len();
    sim_state.commands.drain(0..len);
}

pub fn constrain_particle_in_window(
    particle: Res<Particle>,
    mut entity_query: Query<(&mut Transform, &mut ParticleEntity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width()/2.;
    let height = window.height()/2.;
    let radius = particle.radius;
    let dampening = particle.dampening;
    let new_y_pos = height - radius;
    let new_x_pos = width - radius;
    for (mut trans, mut entity) in entity_query.iter_mut() {

        let lower_x = trans.translation.x - radius;
        let upper_x = trans.translation.x + radius;
        let lower_y = trans.translation.y - radius;
        let upper_y = trans.translation.y + radius;
        if upper_y > height {
            entity.velocity.y = -1. * entity.velocity.y.abs() * dampening;
            trans.translation.y = new_y_pos;
        } else if lower_y < -height {
            entity.velocity.y = entity.velocity.y.abs() * dampening;
            trans.translation.y = -new_y_pos;
        }
        if upper_x > width {
            entity.velocity.x = entity.velocity.x.abs() * -1. * dampening;
            trans.translation.x = new_x_pos;
        } else if lower_x < -width {
            trans.translation.x = -new_x_pos;
            entity.velocity.x = entity.velocity.x.abs() * dampening;
        }
    }
}