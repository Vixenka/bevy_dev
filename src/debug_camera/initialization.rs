use bevy::prelude::*;

use super::{DebugCamera, DebugCameraData, DebugCameraGlobalData};

#[allow(clippy::type_complexity)]
pub(super) fn system(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform, &Transform)>,
    to_initialize: Query<
        (
            Entity,
            &DebugCamera,
            Option<&GlobalTransform>,
            Option<&Transform>,
        ),
        (Added<DebugCamera>, Without<DebugCameraData>),
    >,
    mut global: ResMut<DebugCameraGlobalData>,
) {
    for (entity, debug_camera, global_transform, transform) in to_initialize.iter() {
        let active_camera = cameras.iter().find(|x| x.0.is_active);
        let mut e = commands.get_entity(entity).unwrap();

        let id = global.next_id;
        global.next_id += 1;

        // Set default transforms
        let global_transform = match global_transform {
            Some(global_transform) => *global_transform,
            None => match &active_camera {
                Some(a) => *a.1,
                None => GlobalTransform::default(),
            },
        };
        let transform = match transform {
            Some(transform) => *transform,
            None => match &active_camera {
                Some(a) => *a.2,
                None => Transform::default(),
            },
        };

        // Set index of new camera
        if !debug_camera.focus {
            let pos = global.last_used_debug_cameras.len() - 1;
            global.last_used_debug_cameras.insert(pos, entity);
        }

        // Insert new components
        e.insert((
            Camera3dBundle {
                camera: Camera {
                    is_active: false,
                    ..Default::default()
                },
                global_transform,
                transform,
                ..Default::default()
            },
            DebugCameraData {
                id,
                last_change_position_time: 0.0,
                current_speed: debug_camera.base_speed,
                speed_level: 0.0,
            },
        ));

        // Notify
        bevy::log::info!("Spawned new 3D debug camera #{}", id);
    }
}
