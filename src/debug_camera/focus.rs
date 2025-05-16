use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[cfg(feature = "ui")]
use super::ui::PreviewCamera;
#[cfg(feature = "ui")]
use crate::ui::popup::{PopupEvent, PopupPosition};

use super::{
    DebugCamera, DebugCameraData, DebugCameraGlobalData, DebugCameraLastUsedOriginCameraData,
};

#[allow(clippy::type_complexity)]
pub(super) fn run_if_changed(
    cameras: Query<(), Or<(Added<DebugCamera>, Changed<DebugCamera>)>>,
) -> bool {
    !cameras.is_empty()
}

#[allow(clippy::type_complexity)]
pub(super) fn system(
    #[cfg(not(feature = "ui"))] mut cameras: Query<(
        Entity,
        &mut Camera,
        Option<&mut DebugCamera>,
        Option<&DebugCameraData>,
    )>,
    #[cfg(feature = "ui")] mut cameras: Query<
        (
            Entity,
            &mut Camera,
            Option<&mut DebugCamera>,
            Option<&DebugCameraData>,
        ),
        Without<PreviewCamera>,
    >,
    mut global: ResMut<DebugCameraGlobalData>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    let mut is_any_debug_camera_active = false;
    for (entity, mut camera, mut debug_camera, data) in cameras
        .iter_mut()
        .filter(|x| x.2.is_some() && x.3.is_some())
        .map(|x| (x.0, x.1, x.2.unwrap(), x.3.unwrap()))
    {
        if debug_camera.is_changed() && debug_camera.focus {
            is_any_debug_camera_active = true;

            // Skip if camera is already active
            if camera.is_active {
                continue;
            }

            // Active debug camera
            camera.is_active = true;

            // Set last used debug camera
            for (i, e) in global.last_used_debug_cameras.iter().enumerate() {
                if *e == entity {
                    global.last_used_debug_cameras.remove(i);
                    break;
                }
            }
            global.last_used_debug_cameras.push(entity);

            // Notify user
            let id = data.id;

            bevy::log::info!("Switched to debug camera #{}", id);
            #[cfg(feature = "ui")]
            popup_event.write(PopupEvent::new(
                PopupPosition::BelowCenter,
                1.0,
                move |ui| {
                    ui.strong(format!("Switched to debug camera #{}", id));
                },
            ));

            continue;
        } else if debug_camera.focus {
            // Deactive debug camera
            debug_camera.bypass_change_detection().focus = false;
        }

        camera.is_active = false;
    }

    if is_any_debug_camera_active {
        let Ok(mut primary_window) = window.single_mut() else {
            error!("Expected primary window to exist");
            return;
        };

        // Deactive game camera
        for (entity, mut camera, _, _) in cameras
            .iter_mut()
            .filter(|x| x.2.is_none() && x.1.is_active)
        {
            global.last_used_origin_camera = Some(DebugCameraLastUsedOriginCameraData {
                camera: entity,
                cursor: primary_window.cursor_options.clone(),
            });

            camera.is_active = false;
        }

        // Lock cursor
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    } else {
        // Switch to game camera if no debug camera is active
        if let Some(last) = global.last_used_origin_camera.take() {
            // Activate previous game camera
            if let Ok(mut camera) = cameras.get_mut(last.camera) {
                camera.1.is_active = true;
            }

            // Set cursor
            let Ok(mut primary_window) = window.single_mut() else {
                error!("Expected primary window to exist");
                return;
            };
            primary_window.cursor_options = last.cursor;

            // Notify user
            bevy::log::info!("Switched to game camera");
            #[cfg(feature = "ui")]
            popup_event.write(PopupEvent::new(
                PopupPosition::BelowCenter,
                1.0,
                move |ui| {
                    ui.strong("Switched to game camera");
                },
            ));
        }
    }
}
