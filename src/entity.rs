use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::random;
use crate::resources::{PhysicRules, SimulationState};


pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gravity)
            .add_systems(Update, update_particle_position);
    }
}


pub fn spawn_particle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    amount: u64,
) {
    let window = window_query.get_single().unwrap();

    fn heatmap_color(value: f32) -> (f32, f32, f32) {
        let hue = (1. - value) * 360.;
        let saturation = 0.5;
        let light = 0.5;
        (hue, saturation, light)
    }
    let mass_max = 10000000.;
    for _ in 0..amount
    {
        let r = 20.;

        let mass = random::<f32>() * mass_max;
        let (hue, sat, li) = heatmap_color(mass / mass_max);
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let entity = Entity {
            radius: r,
            ..default()
        };
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(entity.radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::hsl(hue, sat, li))),
                transform: Transform::from_translation(Vec3::new(random_x, random_y, 0.)),
                ..default()
            },
            entity,
        ));
    }
}

#[derive(Component, Default, Debug)]
pub struct Entity {
    pub radius: f32,
    pub velocity: Vec3,
}

pub fn gravity(
    mut entity_query: Query<(&mut Entity)>,
    physic_rules: Res<PhysicRules>,
    time: Res<Time>,
) {
    for (mut entity) in entity_query.iter_mut() {
        entity.velocity += physic_rules.gravity * time.delta_seconds()
    }
}

pub fn update_particle_position(
    mut entity_query: Query<(&mut Transform, &Entity)>,
    simulation_state: Res<SimulationState>,
    time: Res<Time>,
) {
    if !simulation_state.freeze {
        return;
    }
    for (mut trans, mut entity) in entity_query.iter_mut() {
        trans.translation += entity.velocity * time.delta_seconds();
    }
}


pub fn draw_vector(
    mut gizmos: Gizmos,
    mut entity_query: Query<(&Transform, &Entity)>,
) {
    for (tran, entity) in entity_query.iter() {
        gizmos.line(tran.translation, tran.translation + entity.velocity, Color::RED);
    }
}

