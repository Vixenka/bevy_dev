use std::{f32::consts::PI, fmt::Debug, ops::RangeInclusive};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::{Cursor, CursorGrabMode, PrimaryWindow},
};

const MOUSE_LOOK_X_LIMIT: f32 = PI / 2.0;
const RESET_SPEED_THRESHOLD_IN_SECONDS: f32 = 0.2;

#[derive(Default)]
pub struct DebugCameraPlugin {
    pub spawner: DebugCameraSpawner,
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugCameraGlobalData>()
            .add_systems(Update, (initialization, controller));

        let active_spawner = match self.spawner {
            DebugCameraSpawner::Default => {
                #[cfg(not(debug_assertions))]
                bevy::log::warn!("Spawner from bevy_dev's `DebugCamera` is active in release builds. This allows the player to easily activate and manage debug cameras, set the `DebugCameraSpawner` value explicitly in the `DebugCameraPlugin`");
                true
            }
            DebugCameraSpawner::Active => true,
            DebugCameraSpawner::Disabled => false,
        };
        if active_spawner {
            app.add_systems(Update, spawner);
        }
    }
}

#[derive(Default)]
pub enum DebugCameraSpawner {
    #[default]
    Default,
    Active,
    Disabled,
}

#[derive(Debug, Resource, Default)]
pub struct DebugCameraGlobalData {
    pub default_value: DebugCamera,
    pub last_used_origin_camera: Option<DebugCameraLastUsedOriginCameraData>,
}

#[derive(Debug)]
pub struct DebugCameraLastUsedOriginCameraData {
    pub camera: Entity,
    pub cursor: Cursor,
}

#[derive(Component, Debug, Clone)]
#[non_exhaustive]
pub struct DebugCamera {
    pub speed_increase: f32,
    pub speed_multiplier: f32,
    pub speed_multiplier_range: RangeInclusive<f32>,
    pub sensitivity: f32,
    pub base_speed: f32,
    pub focus: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        Self {
            speed_increase: 0.2,
            speed_multiplier: 1.0,
            speed_multiplier_range: 0.001..=2.0,
            sensitivity: 0.001,
            base_speed: 4.5,
            focus: true,
        }
    }
}

#[derive(Debug, Component)]
struct DebugCameraData {
    rotation: Vec2,
    last_change_position_time: f32,
    current_speed: f32,
}

#[derive(Default, Bundle)]
pub struct DebugCamera3dBundle {
    pub camera: Camera3dBundle,
    pub debug_camera: DebugCamera,
}

fn spawner(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if !keys.just_pressed(KeyCode::F1) || !keys.pressed(KeyCode::ShiftLeft) {
        return;
    }

    bevy::log::info!("Spawned 3D debug camera");
    commands.spawn(DebugCamera::default());
}

#[allow(clippy::type_complexity)]
fn initialization(
    mut commands: Commands,
    mut cameras: Query<(
        Entity,
        &mut Camera,
        &GlobalTransform,
        &Transform,
        Option<&DebugCamera>,
    )>,
    mut entities: Query<
        (
            Entity,
            &DebugCamera,
            Option<&GlobalTransform>,
            Option<&Transform>,
        ),
        Added<DebugCamera>,
    >,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut global: ResMut<DebugCameraGlobalData>,
) {
    for (entity, debug_camera, global_transform, transform) in entities.iter_mut() {
        let active_camera = cameras.iter_mut().find(|x| x.1.is_active);
        let mut e = commands.get_entity(entity).unwrap();

        // Set default transforms
        let global_transform = match global_transform {
            Some(global_transform) => *global_transform,
            None => match &active_camera {
                Some(a) => *a.2,
                None => GlobalTransform::default(),
            },
        };
        let transform = match transform {
            Some(transform) => *transform,
            None => match &active_camera {
                Some(a) => *a.3,
                None => Transform::default(),
            },
        };
        let rotation = transform.rotation.to_euler(EulerRot::ZYX);
        let rotation = Vec2::new(rotation.2, rotation.1);

        // Deactive origin camera, and manage window
        if debug_camera.focus {
            if let Some(mut active_camera) = active_camera {
                if active_camera.4.is_none() {
                    let mut window = window
                        .get_single_mut()
                        .expect("Expected primary window to exist");

                    global.last_used_origin_camera = Some(DebugCameraLastUsedOriginCameraData {
                        camera: active_camera.0,
                        cursor: window.cursor,
                    });

                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
                active_camera.1.is_active = false;
            }
        }

        // Insert new components
        e.insert((
            Camera3dBundle {
                camera: Camera {
                    is_active: debug_camera.focus,
                    ..Default::default()
                },
                global_transform,
                transform,
                ..Default::default()
            },
            DebugCameraData {
                rotation,
                last_change_position_time: 0.0,
                current_speed: debug_camera.base_speed,
            },
        ));
    }
}

fn controller(
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
        bevy::log::info!(
            "Speed multiplier: {} {}",
            debug_camera.speed_multiplier,
            input.y
        );
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
        translation += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyE) {
        translation -= Vec3::Y;
    }

    transform.translation += translation.normalize_or_zero()
        * (data.current_speed * debug_camera.speed_multiplier * time.delta_seconds());

    // Rotation
    for input in mouse_motion.read() {
        let x = (data.rotation.x - input.delta.y * debug_camera.sensitivity)
            .clamp(-MOUSE_LOOK_X_LIMIT, MOUSE_LOOK_X_LIMIT);
        let y = data.rotation.y - input.delta.x * debug_camera.sensitivity;

        data.rotation = Vec2::new(x, y);
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, data.rotation.y, data.rotation.x);
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
