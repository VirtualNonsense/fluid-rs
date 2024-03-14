use std::vec;
use bevy::app::App;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::tracing::field::debug;
use bevy::window::PrimaryWindow;
use crate::particle::{Particle, spawn_particle, ParticleEntity, SpawnSetting};
use crate::resources::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowState>();
        app.add_systems(Update, handle_simulation_commands);
        app.add_systems(Update, constrain_particle_in_window);
        app.add_systems(Update, update_window_speed);
        // app.add_systems(Update, draw_vector);

    }
}

pub fn draw_vector(
    mut gizmos: Gizmos,
    sim: ResMut<WindowState>,
) {
    let pos = vec2(0., 0.);
    let dir = sim.window_speed.normalize();
    let length = sim.window_speed.length();
    let head = pos + length * dir;
    let arrow_wing1 = length/2. * dir.rotate(Vec2::from_angle(f32::to_radians(45.)));
    let arrow_wing2 = length/2. *  dir.rotate(Vec2::from_angle(f32::to_radians(-45.)));
    gizmos.line_2d(pos, head, Color::RED);
    gizmos.line_2d(head, arrow_wing1, Color::RED);
    gizmos.line_2d(head, arrow_wing2, Color::RED);
}

#[derive(Default, Debug, Resource)]
pub struct WindowState {
    pub last_position: Option<Vec2>,
    pub window_speed: Vec2,
    pub delta: Vec2,
}

fn update_window_speed(
    mut sim: ResMut<WindowState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    match window.position {
        WindowPosition::At(vec) => {
            if let Some(last_pos) = sim.last_position {
                sim.delta = vec.as_vec2() - last_pos;
                sim.window_speed = -sim.delta / time.delta_seconds();
                sim.window_speed.x *= -1.;
                sim.last_position = Some(vec.as_vec2());
                return;
            }
            sim.window_speed = Vec2::new(0.0, 0.0);
            sim.delta = Vec2::new(0.0, 0.0);
            sim.last_position = Some(vec.as_vec2());
            return;
        }
        _ => {}
    }
    sim.delta = Vec2::new(0.0, 0.0);
    sim.window_speed = Vec2::new(0.0, 0.0);
    sim.last_position = None;
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
        if particle_count > 1 {
            println!("adding {} new particle!", particle_count);
        } else {
            println!("adding a new particle!");
        }
        spawn_particle(
            commands,
            window_query,
            meshes,
            materials,
            particle_count,
            particle.radius,
            SpawnSetting::Random,
        );
    }
    let len = sim_state.commands.len();
    sim_state.commands.drain(0..len);
}

pub fn constrain_particle_in_window(
    particle: Res<Particle>,
    mut entity_query: Query<(&mut Transform, &mut ParticleEntity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    sim: Res<WindowState>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width() / 2.;
    let height = window.height() / 2.;
    let radius = particle.radius;
    let dampening = particle.dampening;
    let new_y_pos = height - radius;
    let new_x_pos = width - radius;
    for (mut trans, mut entity) in entity_query.iter_mut() {
        trans.translation.x -= sim.delta.x;
        trans.translation.y += sim.delta.y;
        let lower_x = trans.translation.x - radius;
        let upper_x = trans.translation.x + radius;
        let lower_y = trans.translation.y - radius;
        let upper_y = trans.translation.y + radius;
        if upper_y > height {
            trans.translation.y = new_y_pos;
            entity.velocity.y = -1. * entity.velocity.y.abs() * dampening;
            entity.velocity.y += sim.window_speed.y;
        } else if lower_y < -height {
            trans.translation.y = -new_y_pos;
            entity.velocity.y = entity.velocity.y.abs() * dampening;
            entity.velocity.y += sim.window_speed.y;
        }
        if upper_x > width {
            trans.translation.x = new_x_pos;
            entity.velocity.x = -1. * entity.velocity.x.abs() * dampening;
            entity.velocity.x += sim.window_speed.x;
        } else if lower_x < -width {
            trans.translation.x = -new_x_pos;
            entity.velocity.x = entity.velocity.x.abs() * dampening;
            entity.velocity.x += sim.window_speed.x;
        }
    }
}