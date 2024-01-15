#[cfg(test)]
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod tests {
    #[test]
    fn main() {
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
    }


fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("hi im speaking").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
}
