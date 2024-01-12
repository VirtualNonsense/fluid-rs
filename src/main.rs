mod entity;
mod camera;
mod particle;
mod user_interface;
mod resources;
mod simulation;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};

use crate::entity::EntityPlugin;
use crate::camera::CameraPlugin;
use crate::resources::ResourcePlugin;
use crate::simulation::SimulationPlugin;
use crate::user_interface::UserInterfacePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(CameraPlugin)
        .add_plugins(ResourcePlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UserInterfacePlugin)
        .add_plugins(EntityPlugin)
        .run();
}