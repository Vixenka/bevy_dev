use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[cfg(feature = "ui")]
use crate::ui::popup::{PopupEvent, PopupPosition};

use super::{DebugCamera, DebugCameraControls, DebugCameraData};

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
    controls: Res<DebugCameraControls>,
    #[cfg(feature = "ui")] mut popup_event: EventWriter<PopupEvent>,
) {
    let (mut transform, mut data, mut debug_camera, _) =
        match cameras.iter_mut().find(|x| x.3.is_active) {
            Some(v) => v,
            None => return,
        };

    // Speed
    for input in mouse_wheel.read() {
        data.speed_level += input.y;
        data.speed_level = data.speed_level.clamp(
            (debug_camera.speed_multiplier_range.start().log2() * 4.0).floor(),
            (debug_camera.speed_multiplier_range.end().log2() * 4.0).ceil(),
        );

        debug_camera.speed_multiplier = 2.0f32.powf(data.speed_level * 0.25).clamp(
            *debug_camera.speed_multiplier_range.start(),
            *debug_camera.speed_multiplier_range.end(),
        );

        #[cfg(feature = "ui")]
        {
            let value = debug_camera.speed_multiplier;
            popup_event.write(PopupEvent::new(
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
    if keys.pressed(controls.move_forward) {
        translation -= Vec3::from(transform.local_z());
    }
    if keys.pressed(controls.move_backward) {
        translation += Vec3::from(transform.local_z());
    }
    if keys.pressed(controls.move_left) {
        translation -= Vec3::from(transform.local_x());
    }
    if keys.pressed(controls.move_right) {
        translation += Vec3::from(transform.local_x());
    }
    if keys.pressed(controls.move_up) {
        translation += Vec3::Y;
    }
    if keys.pressed(controls.move_down) {
        translation -= Vec3::Y;
    }

    transform.translation += translation.normalize_or_zero()
        * (data.current_speed * debug_camera.speed_multiplier * time.delta_secs());

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
        data.current_speed += data.current_speed * time.delta_secs() * debug_camera.speed_increase;
        data.last_change_position_time = time.elapsed_secs();
    } else if data.last_change_position_time > RESET_SPEED_THRESHOLD_IN_SECONDS {
        data.current_speed = debug_camera.base_speed;
    }
}
