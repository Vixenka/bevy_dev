/*!
 * Helpers for creating UI elements.
 *
 * Require feature `ui` to be enabled.
 */

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui::Ui};

use crate::{prelude::DebugCameraActive, ui::popup::PopupEvent};

pub mod popup;

/// Plugin for the debug UI.
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin::default());
        }

        app.add_systems(PostUpdate, initialize_ctx)
            .add_plugins(popup::PopupPlugin);
    }
}

/// Creates a popup of ui on screen without need to write handly event.
/// Default set its on the center of screen, with duration time of 5 seconds.
pub fn popup(ui: impl Fn(&mut Ui) + Send + Sync + 'static) -> PopupEvent {
    PopupEvent::new(popup::PopupPosition::default(), 5.0, ui).fetch()
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UiContextPass;

fn initialize_ctx(
    mut removed: RemovedComponents<DebugCameraActive>,
    added: Single<Entity, Added<DebugCameraActive>>,
    mut commands: Commands,
) {
    for entity in removed.read() {
        if let Ok(mut commands) = commands.get_entity(entity) {
            commands
                .remove::<EguiContext>()
                .remove::<EguiMultipassSchedule>();
        }
    }

    commands.get_entity(added.entity()).unwrap().insert((
        EguiContext::default(),
        EguiMultipassSchedule::new(UiContextPass),
    ));
}
