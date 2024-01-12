use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2d};
use bevy::window::PrimaryWindow;
use rand::random;
use crate::resources::{PhysicRules, SimulationState};


pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Particle>()
            .add_systems(Update, gravity)
            .add_systems(Update, update_particle_position)
            .add_systems(Update, change_material)
            .add_systems(Update, collide_particles);
    }
}


fn heatmap_color(value: f32) -> (f32, f32, f32) {
    let hue = (1. - value) * 360.;
    let saturation = 0.5;
    let light = 0.5;
    (hue, saturation, light)
}

pub fn spawn_particle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    amount: u64,
    radius: f32,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..amount
    {

        let (hue, sat, li) = heatmap_color(0.);
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let entity = VelocityEntity {
            ..default()
        };
        let b = MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hsl(hue, sat, li))),
            transform: Transform::from_translation(Vec3::new(random_x, random_y, 0.)),
            ..default()
        };

        commands.spawn((
            b,
            entity,
        ));
    }
}

#[derive(Component, Default, Debug)]
pub struct VelocityEntity {
    pub velocity: Vec3,
}

#[derive(Resource, Debug)]
pub struct Particle {
    pub radius: f32,
    pub dampening: f32,
    pub mass: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            radius: 10.,
            dampening: 0.9,
            mass: 1.,
        }
    }
}

pub fn gravity(
    simulation_state: Res<SimulationState>,
    mut entity_query: Query<(&mut VelocityEntity)>,
    physic_rules: Res<PhysicRules>,
    time: Res<Time>,
) {
    if simulation_state.freeze {
        return;
    }
    for (mut entity) in entity_query.iter_mut() {
        entity.velocity += physic_rules.gravity * time.delta_seconds()
    }
}

pub fn update_particle_position(
    mut entity_query: Query<(&mut Transform, &VelocityEntity)>,
    simulation_state: Res<SimulationState>,
    time: Res<Time>,
) {
    if simulation_state.freeze {
        return;
    }
    for (mut trans, mut entity) in entity_query.iter_mut() {
        trans.translation += entity.velocity * time.delta_seconds();
    }
}

fn change_material(
    enemies: Query<(&Handle<ColorMaterial>, &VelocityEntity)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (handle, part) in enemies.iter() {
        let color = &mut materials.get_mut(handle).unwrap().color;
        let speed = part.velocity.length() / 300.;

        let (hue, sat, lig) = heatmap_color(speed);

        color.set_h(hue);
        color.set_s(sat);
        color.set_s(lig);
    }
}


pub fn draw_vector(
    mut gizmos: Gizmos,
    mut entity_query: Query<(&Transform, &VelocityEntity)>,
) {
    for (tran, entity) in entity_query.iter() {
        gizmos.line(tran.translation, tran.translation + entity.velocity, Color::RED);
    }
}

pub fn collide_particles(
    simulation_state: Res<SimulationState>,
    particle: Res<Particle>,
    mut entity_query: Query<(&mut Transform, &mut VelocityEntity)>) {
    if simulation_state.freeze {
        return;
    }

    let mut combinations = entity_query.iter_combinations_mut();
    while let Some([(mut tran_a, mut enti_a),
                   (mut tran_b, mut enti_b)]) = combinations.fetch_next() {
        let dist = tran_a.translation.distance(tran_b.translation);
        let delta = 2.*particle.radius - dist;
        if delta > 0.0 {
            let direction = (tran_a.translation - tran_b.translation).normalize();
            tran_a.translation += direction * delta / 2.;
            tran_b.translation -= direction * delta / 2.;

            let v_a = enti_a.velocity;
            let m_a = particle.mass;
            let v_b = enti_b.velocity;
            let m_b = particle.mass;

            let va_n = v_a.project_onto(-direction);
            let va_t = v_a - va_n;
            let vb_n = v_b.project_onto(direction);
            let vb_t = v_b - vb_n;

            let va_n_new = (m_a * va_n + m_b * (2. * vb_n - va_n)) / (m_a + m_b) * particle.dampening;
            let vb_n_new = (m_b * vb_n + m_a * (2. * va_n - vb_n)) / (m_a + m_b) * particle.dampening;

            enti_a.velocity = va_n_new + va_t;
            enti_b.velocity = vb_n_new + vb_t;
        }
    }
}