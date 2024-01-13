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
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
use crate::particle::Particle;
use crate::resources::{ PhysicRules, SimulationState, SimulationTrigger};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UIState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, initialize_user_interface)
            .add_systems(Startup, initialize_ui_state)
            .add_systems(Update, draw_ui)
            .add_systems(Update, update_menu_state)
        ;
    }
}

#[derive(Debug, Resource)]
struct UIState {
    show_menu: bool,
    spawn_counter: u64,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            show_menu: true,
            spawn_counter: 0
        }
    }
}

fn update_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UIState>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) {
        ui_state.show_menu = !ui_state.show_menu;
    }
}

fn initialize_user_interface(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn initialize_ui_state(mut ui_state: ResMut<UIState>) {
    // ui_state.is_window_open = true;
}

fn draw_ui(
    mut ui_state: ResMut<UIState>,
    mut sim_state: ResMut<SimulationState>,
    mut phy_rules: ResMut<PhysicRules>,
    mut particle: ResMut<Particle>,
    mut contexts: EguiContexts,
) {
    if !ui_state.show_menu {
        return;
    }
    let ctx = contexts.ctx_mut();
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.checkbox(&mut sim_state.freeze, "Freeze");
            if ui.button("Reset simulation").clicked() {
                if !sim_state.trigger.contains(&SimulationTrigger::Reset) {
                    sim_state.trigger.push(SimulationTrigger::Reset);
                }
            };
            if ui.button("Add Particle").clicked() {
                sim_state.trigger.push(SimulationTrigger::AddParticle(ui_state.spawn_counter));
            }
            ui.add(egui::Slider::new(&mut ui_state.spawn_counter, 1..=500).text("particles"));
            let max = 10.;
            let min = -10.;
            ui.add(egui::Slider::new(&mut phy_rules.gravity.x, min..=max).text("gravity x"));
            ui.add(egui::Slider::new(&mut phy_rules.gravity.y, min..=max).text("gravity y"));
            // ui.add(egui::Slider::new(&mut phy_rules.gravity.z, min..=max).text("gravity z"));
            ui.add(egui::Slider::new(&mut particle.dampening, 0.0..=1.).text("dampening"));
            if ui.add(egui::Slider::new(&mut particle.radius, 0.1..=20.).text("radius")).changed() {
                sim_state.trigger.push(SimulationTrigger::ChangeParticleScale(particle.radius));
            };
        });
}

