use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[cfg(feature = "ui")]
use crate::ui::popup::{PopupEvent, PopupPosition};

use super::{DebugCamera, DebugCameraData};

const MOUSE_LOOK_X_LIMIT: f32 = PI / 2.0;
const RESET_SPEED_THRESHOLD_IN_SECONDS: f32 = 0.2;

pub(super) fn system(
    mut cameras: Query<(
        &mut Transform,
        &mut DebugCameraData,
        &mut DebugCamera,
        &Camera,
    )>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    let (mut transform, mut data, mut debug_camera, _) =
        match cameras.iter_mut().find(|x| x.3.is_active) {
            Some(v) => v,
            None => return,
        };

    // Speed
    for input in mouse_wheel.read() {
        debug_camera.speed_multiplier = (debug_camera.speed_multiplier
            * match input.y > 0.0 {
                true => 1.1,
                false => 0.9,
            })
        .clamp(
            *debug_camera.speed_multiplier_range.start(),
            *debug_camera.speed_multiplier_range.end(),
        );

        #[cfg(feature = "ui")]
        {
            let value = debug_camera.speed_multiplier;
            popup_event.send(PopupEvent::new(
                PopupPosition::BelowCenter,
                0.5,
                move |ui| {
                    ui.label(format!("Speed multiplier: {:.2}", value));
                },
            ));
        }
    }

    // Position
    let mut translation = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        translation -= Vec3::from(transform.local_z());
    }
    if keys.pressed(KeyCode::KeyS) {
        translation += Vec3::from(transform.local_z());
    }
    if keys.pressed(KeyCode::KeyA) {
        translation -= Vec3::from(transform.local_x());
    }
    if keys.pressed(KeyCode::KeyD) {
        translation += Vec3::from(transform.local_x());
    }
    if keys.pressed(KeyCode::KeyQ) {
        translation -= Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyE) {
        translation += Vec3::Y;
    }

    transform.translation += translation.normalize_or_zero()
        * (data.current_speed * debug_camera.speed_multiplier * time.delta_seconds());

    // Rotation
    for input in mouse_motion.read() {
        let (mut y, mut x, _) = transform.rotation.to_euler(EulerRot::YXZ);

        x -= (input.delta.y * debug_camera.sensitivity).to_radians();
        x = x.clamp(-MOUSE_LOOK_X_LIMIT, MOUSE_LOOK_X_LIMIT);

        y -= (input.delta.x * debug_camera.sensitivity).to_radians();

        transform.rotation = Quat::from_rotation_y(y) * Quat::from_rotation_x(x);
    }

    // Increase speed
    if translation != Vec3::ZERO {
        data.current_speed +=
            data.current_speed * time.delta_seconds() * debug_camera.speed_increase;
        data.last_change_position_time = time.elapsed_seconds();
    } else if data.last_change_position_time > RESET_SPEED_THRESHOLD_IN_SECONDS {
        data.current_speed = debug_camera.base_speed;
    }
}
