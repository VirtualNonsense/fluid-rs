use bevy::app::{App, Plugin, Startup};
use bevy::core_pipeline::bloom::BloomSettings;

use bevy::prelude::{Camera, Camera2dBundle, Commands, default, Query, Window, With};

use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings {
            intensity: 0.4,
            high_pass_frequency: 0.8,
            low_frequency_boost: 0.8,
            ..default()
        });
}