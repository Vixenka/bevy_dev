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
        app.add_plugins(bevy_egui::EguiPlugin)
            .add_plugins(popup::PopupPlugin);
    }
}
