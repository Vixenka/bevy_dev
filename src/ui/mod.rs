use bevy::prelude::*;

pub mod popup;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(popup::PopupPlugin);
    }
}
