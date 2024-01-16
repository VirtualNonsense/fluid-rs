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
use bevy::math::vec3;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
use crate::particle::Particle;
use crate::resources::{PhysicRules, SimulationState, SimulationCommands};

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
            spawn_counter: 0,
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
    mut contexts: EguiContexts,

    mut sim_state: ResMut<SimulationState>,
    physic_rules: Res<PhysicRules>,
    particle: Res<Particle>,
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
                if !sim_state.commands.contains(&SimulationCommands::DeleteAllParticles) {
                    sim_state.commands.push(SimulationCommands::DeleteAllParticles);
                }
            };
            if ui.button("Add Particle").clicked() {
                sim_state.commands.push(SimulationCommands::AddParticle(ui_state.spawn_counter));
            }
            ui.add(egui::Slider::new(&mut ui_state.spawn_counter, 1..=500).text("particles"));
            let max = 10.;
            let min = -10.;
            let mut gravity = physic_rules.gravity;
            let mut gravity_changed = false;
            let mut dampening = particle.dampening;
            let mut radius = particle.radius;
            let mut mass = particle.mass;
            if ui.add(egui::Slider::new(&mut gravity.x, min..=max).text("gravity x")).changed() {
                gravity_changed = true;
            };
            if ui.add(egui::Slider::new(&mut gravity.y, min..=max).text("gravity y")).changed() {
                gravity_changed = true;
            };
            if gravity_changed {
                sim_state.commands.push(SimulationCommands::ChangeGravity(gravity));
            }

            if ui.add(egui::Slider::new(&mut dampening, 0.0..=1.).text("dampening")).changed() {
                sim_state.commands.push(SimulationCommands::ChangeParticleDampening(dampening));
            };
            if ui.add(egui::Slider::new(&mut radius, 0.1..=20.).text("radius")).changed() {
                sim_state.commands.push(SimulationCommands::ChangeParticleScale(radius));
            };
            if ui.add(egui::Slider::new(&mut mass, 0.1..=200.).text("mass")).changed(){
                sim_state.commands.push(SimulationCommands::ChangeParticleMass(mass));
            }
        });
}

