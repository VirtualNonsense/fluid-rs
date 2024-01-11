use bevy::prelude::*;

const GRAVITY: Vec3 = Vec3 {
    x: 0.0,
    y: -90.8,
    z: 0.0,
};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gravity)
            .add_systems(Update, update_particle_position);
    }
}


#[derive(Component, Default, Debug)]
pub struct Entity {
    pub velocity: Vec3,
}

pub fn gravity(
    mut entity_query: Query<(&mut Entity)>,
    time: Res<Time>,
) {
    for (mut entity) in entity_query.iter_mut() {
        entity.velocity += GRAVITY * time.delta_seconds()
    }
}

pub fn update_particle_position(
    mut entity_query: Query<(&mut Transform, &Entity)>,
    time: Res<Time>,
) {
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

