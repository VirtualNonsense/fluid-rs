use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
    app::App,
};

pub struct UserInterfacePlugin {}

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems()

    }
}

