use bevy::prelude::*;
use bevy_egui::egui::Color32;

use crate::ui::popup::{PopupEvent, PopupPosition};

use super::{DebugCamera, DebugCameraData, DebugCameraGlobalData};

pub(super) fn debug_camera_selector_ui(
    debug_cameras: &mut Query<(Entity, &mut DebugCamera, &DebugCameraData)>,
    global: &mut ResMut<DebugCameraGlobalData>,
    popup_event: &mut EventWriter<PopupEvent>,
) {
    let mut data = Vec::new();
    for entity in global.last_used_debug_cameras.iter() {
        let camera = debug_cameras.get_mut(*entity).unwrap();
        data.push(camera.2.id);
    }

    let selected_camera = global.selected_camera.unwrap();

    popup_event.send(PopupEvent::new(PopupPosition::Center, 0.0, move |ui| {
        ui.strong("Select debug camera:");
        ui.horizontal_wrapped(|ui| {
            for (i, entity) in data.iter().enumerate().rev() {
                ui.colored_label(
                    match selected_camera == i {
                        true => Color32::RED,
                        false => Color32::WHITE,
                    },
                    format!("#{}", entity),
                );
            }
        });
    }));
}
