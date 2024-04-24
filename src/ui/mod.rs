/*!
 * Helpers for creating UI elements.
 *
 * Require feature `ui` to be enabled.
 */

use bevy::prelude::*;

pub mod popup;

/// Plugin for the debug UI.
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }

        app.add_plugins(popup::PopupPlugin);
    }
}
